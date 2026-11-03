from pathlib import Path
from hashlib import sha256
from PIL import Image
import json

root = Path(__file__).resolve().parents[1]
target = root / 'design-qa/final'
functional = {}
for name, filename in [('core', 'functional-run.log'), ('vault', 'vault-run.log'), ('edge', 'edge-run.log'), ('production', 'production-smoke.log')]:
    lines = (root / 'output/phase2-native' / filename).read_text(encoding='utf-8-sig').splitlines()
    index = next(i for i, line in enumerate(lines) if line == '### Result')
    functional[name] = json.loads(lines[index + 1])
(target / 'functional-results.json').write_text(json.dumps(functional, indent=2, ensure_ascii=False), encoding='utf-8')
manifest = []
for name in ('recent', 'images', 'passwords', 'settings'):
    for kind in ('reference', 'implementation', 'overlay', 'diff'):
        path = target / name / f'{kind}.png'
        assert path.read_bytes().startswith(b'\x89PNG\r\n\x1a\n')
        with Image.open(path) as image:
            assert image.size == (1586, 992)
        manifest.append({'file': str(path.relative_to(target)), 'size': [1586, 992], 'sha256': sha256(path.read_bytes()).hexdigest()})
(target / 'png-manifest.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
print(json.dumps({'direct_comparison_pngs': len(manifest), 'functional_checks': sum(map(len, functional.values())), 'functional_failed': sum(not r['pass'] for group in functional.values() for r in group)}))
