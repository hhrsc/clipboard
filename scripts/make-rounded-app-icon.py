from math import ceil, floor
from pathlib import Path

from PIL import Image, ImageDraw

root = Path(__file__).resolve().parents[1]
source = Image.open(root / 'assets/app-icon-clip-sprite.png').convert('RGBA')
if source.width != source.height:
    raise ValueError('图标原图必须是正方形')

size = source.width * 4
mask = Image.new('L', (size, size))
draw = ImageDraw.Draw(mask)

# 五次超椭圆与超采样仅修改透明度，保留原图所有 RGB 像素。
for y in range(size):
    distance = abs(2 * (y + 0.5) / size - 1)
    half_width = (1 - distance**5) ** 0.2 * size / 2
    draw.line((ceil(size / 2 - half_width), y, floor(size / 2 + half_width - 1), y), fill=255)

source.putalpha(mask.resize(source.size, Image.Resampling.LANCZOS))
source.save(root / 'assets/app-icon-clip-sprite-rounded.png')
