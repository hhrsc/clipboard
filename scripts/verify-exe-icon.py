import ctypes
import json
import struct
import sys
from pathlib import Path

from PIL import Image

exe, expected, output = map(Path, sys.argv[1:])
kernel = ctypes.WinDLL('kernel32', use_last_error=True)
pointer = ctypes.c_void_p
kernel.LoadLibraryExW.argtypes = [ctypes.c_wchar_p, pointer, ctypes.c_uint32]
kernel.LoadLibraryExW.restype = pointer
kernel.FindResourceW.argtypes = [pointer, pointer, pointer]
kernel.FindResourceW.restype = pointer
kernel.LoadResource.argtypes = [pointer, pointer]
kernel.LoadResource.restype = pointer
kernel.LockResource.argtypes = [pointer]
kernel.LockResource.restype = pointer
kernel.SizeofResource.argtypes = [pointer, pointer]
kernel.SizeofResource.restype = ctypes.c_uint32
kernel.FreeLibrary.argtypes = [pointer]

# 仅映射 PE 资源，不执行 EXE，避免 Windows 图标缓存影响核对。
module = kernel.LoadLibraryExW(str(exe.resolve()), None, 0x22)
if not module:
    raise ctypes.WinError(ctypes.get_last_error())


def resource(name, kind):
    handle = kernel.FindResourceW(module, name, kind)
    if not handle:
        raise ctypes.WinError(ctypes.get_last_error())
    data = kernel.LockResource(kernel.LoadResource(module, handle))
    if not data:
        raise ctypes.WinError(ctypes.get_last_error())
    return ctypes.string_at(data, kernel.SizeofResource(module, handle))


try:
    group = resource(32512, 14)
    count = struct.unpack_from('<H', group, 4)[0]
    offset = 6 + count * 16
    entries, images = [], []
    for i in range(count):
        width, height, colors, reserved, planes, depth, _, identifier = struct.unpack_from('<BBBBHHIH', group, 6 + i * 14)
        data = resource(identifier, 3)
        entries.append(struct.pack('<BBBBHHII', width, height, colors, reserved, planes, depth, len(data), offset))
        images.append(data)
        offset += len(data)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(group[:6] + b''.join(entries + images))
finally:
    kernel.FreeLibrary(module)

with Image.open(output) as actual, Image.open(expected) as wanted:
    sizes = sorted(actual.ico.sizes())
    matches = {
        str(size): size in wanted.ico.sizes()
        and actual.ico.getimage(size).convert('RGBA').tobytes() == wanted.ico.getimage(size).convert('RGBA').tobytes()
        for size in sizes
    }
    actual.ico.getimage((32, 32)).save(output.with_suffix('.png'))
    same = actual.ico.sizes() == wanted.ico.sizes() and all(matches.values())
    print(json.dumps({'exe': str(exe), 'size_matches': matches, 'matches': same}))
    sys.exit(0 if same else 1)
