use std::io::{Read, Write};
use std::path::Path;
use zeroize::Zeroizing;

const MAX_BLOB_BYTES: usize = 16 * 1024;

#[cfg(windows)]
fn crypt(data: &[u8], context: &[u8], protect: bool) -> Result<Zeroizing<Vec<u8>>, String> {
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::{Foundation::LocalFree, Security::Cryptography::{
        CryptProtectData, CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
    }};
    use zeroize::Zeroize;

    if data.is_empty() || data.len() > MAX_BLOB_BYTES || context.len() > MAX_BLOB_BYTES {
        return Err("Local vault access data is invalid".into());
    }
    let input = CRYPT_INTEGER_BLOB { cbData: data.len() as u32, pbData: data.as_ptr().cast_mut() };
    let entropy = CRYPT_INTEGER_BLOB { cbData: context.len() as u32, pbData: context.as_ptr().cast_mut() };
    let mut output = CRYPT_INTEGER_BLOB { cbData: 0, pbData: null_mut() };
    // 仅使用当前 Windows 用户的 DPAPI，不启用整机共享；释放前擦除系统缓冲区。
    unsafe {
        let ok = if protect {
            CryptProtectData(&input, null(), &entropy, null(), null(), CRYPTPROTECT_UI_FORBIDDEN, &mut output)
        } else {
            CryptUnprotectData(&input, null_mut(), &entropy, null(), null(), CRYPTPROTECT_UI_FORBIDDEN, &mut output)
        };
        if ok == 0 {
            return Err(format!("Windows could not access the local vault key: {}", std::io::Error::last_os_error()));
        }
        if output.pbData.is_null() { return Err("Windows returned no local vault key".into()); }
        let bytes = std::slice::from_raw_parts_mut(output.pbData, output.cbData as usize);
        let result = Zeroizing::new(bytes.to_vec());
        bytes.zeroize();
        LocalFree(output.pbData.cast());
        Ok(result)
    }
}

#[cfg(not(windows))]
fn crypt(_: &[u8], _: &[u8], _: bool) -> Result<Zeroizing<Vec<u8>>, String> {
    Err("Password-free vault access is only available on Windows".into())
}

pub fn save(path: &Path, key: &[u8; 32], context: &[u8]) -> Result<(), String> {
    let protected = crypt(key, context, true)?;
    let verified = crypt(&protected, context, false)?;
    if verified.as_slice() != key { return Err("Local vault key verification failed".into()); }
    let temp = path.with_extension("tmp");
    let result = (|| -> std::io::Result<()> {
        let mut file = std::fs::File::create(&temp)?;
        file.write_all(&protected)?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(&temp, path)
    })();
    if let Err(error) = result {
        let _ = std::fs::remove_file(temp);
        return Err(format!("Could not save local vault access: {error}"));
    }
    Ok(())
}

pub fn read(path: &Path, context: &[u8]) -> Result<Zeroizing<[u8; 32]>, String> {
    let mut bytes = Vec::new();
    std::fs::File::open(path).and_then(|file| file.take(MAX_BLOB_BYTES as u64 + 1).read_to_end(&mut bytes))
        .map_err(|_| "Local vault access is unavailable; use your master password".to_string())?;
    let plaintext = crypt(&bytes, context, false)?;
    let key: [u8; 32] = plaintext.as_slice().try_into().map_err(|_| "Local vault key is invalid".to_string())?;
    Ok(Zeroizing::new(key))
}

pub fn remove(path: &Path) -> Result<(), String> {
    for candidate in [path.with_extension("tmp"), path.to_path_buf()] {
        match std::fs::remove_file(candidate) {
            Ok(()) => (),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(format!("Could not remove local vault access: {error}")),
        }
    }
    Ok(())
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn device_key_round_trip_and_context_binding() {
        let key = [42; 32];
        let blob = crypt(&key, b"synthetic-vault-a", true).unwrap();
        assert!(!blob.windows(32).any(|window| window == key));
        assert_eq!(crypt(&blob, b"synthetic-vault-a", false).unwrap().as_slice(), key);
        assert!(crypt(&blob, b"synthetic-vault-b", false).is_err());
        let mut damaged = blob.to_vec();
        let last = damaged.len() - 1;
        damaged[last] ^= 1;
        assert!(crypt(&damaged, b"synthetic-vault-a", false).is_err());
    }

    #[test]
    fn persisted_key_can_be_replaced_and_removed() {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("clipboard-local-access-test-{stamp}.bin"));
        save(&path, &[42; 32], b"synthetic-vault").unwrap();
        assert_eq!(*read(&path, b"synthetic-vault").unwrap(), [42; 32]);
        save(&path, &[43; 32], b"synthetic-vault").unwrap();
        assert_eq!(*read(&path, b"synthetic-vault").unwrap(), [43; 32]);
        remove(&path).unwrap();
        assert!(!path.exists() && !path.with_extension("tmp").exists());
        assert!(read(&path, b"synthetic-vault").is_err());
        remove(&path).unwrap();
    }
}
