from pathlib import Path
import json
from PIL import Image, ImageChops, ImageEnhance

root = Path(__file__).resolve().parents[1]
metrics = []
for name in ('recent', 'images', 'passwords', 'settings'):
    target = root / 'design-qa' / 'final' / name
    target.mkdir(parents=True, exist_ok=True)
    source = root / 'reference' / f'{name}.png'
    captured = target / 'implementation.png'
    assert captured.read_bytes()[:8] == b'\x89PNG\r\n\x1a\n', f'{name}: not PNG'
    reference = Image.open(source).convert('RGB')
    implementation = Image.open(captured).convert('RGB')
    assert reference.size == implementation.size == (1586, 992)
    reference.save(target / 'reference.png')
    Image.blend(reference, implementation, .5).save(target / 'overlay.png')
    difference = ImageChops.difference(reference, implementation)
    difference.save(target / 'diff.png')
    ImageEnhance.Brightness(difference).enhance(4).save(target / 'diff-amplified.png')
    pair = Image.new('RGB', (3172, 992))
    pair.paste(reference, (0, 0))
    pair.paste(implementation, (1586, 0))
    pair.save(target / 'comparison.png')
    regions = {
        'sidebar': (15, 120, 292, 870),
        'recent': (805, 280, 1565, 470),
        'images': (1120, 90, 1560, 585),
        'passwords': (825, 175, 1557, 495),
        'settings': (1135, 215, 1460, 615),
    }
    for label in ('sidebar', name):
        box = regions[label]
        before, after = reference.crop(box), implementation.crop(box)
        region_pair = Image.new('RGB', (before.width * 2, before.height))
        region_pair.paste(before, (0, 0))
        region_pair.paste(after, (before.width, 0))
        region_pair.save(target / f'comparison-{label}.png')
    pixels = list(difference.get_flattened_data())
    metrics.append({'page': name, 'size': reference.size, 'mean_absolute_rgb_error': round(sum(sum(p) for p in pixels) / (len(pixels) * 3), 4), 'pixels_over_24_rgb_delta_percent': round(sum(max(p) > 24 for p in pixels) / len(pixels) * 100, 4)})
(root / 'design-qa' / 'final' / 'metrics.json').write_text(json.dumps(metrics, indent=2), encoding='utf-8')
print(json.dumps(metrics, indent=2))
