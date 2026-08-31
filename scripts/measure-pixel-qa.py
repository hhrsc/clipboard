from pathlib import Path
from statistics import median
from PIL import Image, ImageChops

root = Path(__file__).resolve().parents[1]
for name in ('recent', 'images', 'passwords', 'settings'):
    images = [Image.open(root / 'reference' / f'{name}.png').convert('RGB'), Image.open(root / 'design-qa/final' / name / 'implementation.png').convert('RGB')]
    print(name)
    for box in ((35,143,58,168), (33,307,60,334), (35,792,59,818), (80,350,200,375), (1500,850,1550,870)):
        values = []
        for image in images:
            pixels = list(image.crop(box).get_flattened_data())
            dark = [(i % (box[2]-box[0])+box[0], i // (box[2]-box[0])+box[1]) for i, pixel in enumerate(pixels) if max(pixel)<160]
            values.append({'median': tuple(median(p[i] for p in pixels) for i in range(3)), 'inkBounds': (min(x for x,y in dark),min(y for x,y in dark),max(x for x,y in dark),max(y for x,y in dark)) if dark else None})
        print(box, values)
    if name == 'images':
        for box in ((1135,110,1540,565),(350,230,500,420)):
            delta = list(ImageChops.difference(images[0].crop(box),images[1].crop(box)).get_flattened_data())
            print('image inner crop', box, sum(sum(p) for p in delta)/len(delta)/3)
