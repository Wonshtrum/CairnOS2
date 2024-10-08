from PIL import Image
from sys import argv

width = 80
height = 25
dark = False
img = Image.open(argv[1]).convert("RGB")

palette = [
    (0x00, 0x00, 0x00),
    (0x00, 0x00, 0x80),
    (0x00, 0x80, 0x00),
    (0x00, 0x80, 0x80),
    (0x80, 0x00, 0x00),
    (0x80, 0x00, 0x80),
    (0x80, 0x80, 0x00),
    (0xC0, 0xC0, 0xC0),
    (0x80, 0x80, 0x80),
    (0x00, 0x00, 0xFF),
    (0x00, 0xFF, 0x00),
    (0x00, 0xFF, 0xFF),
    (0xFF, 0x00, 0x00),
    (0xFF, 0x00, 0xFF),
    (0xFF, 0xFF, 0x00),
    (0xFF, 0xFF, 0xFF),
]

if dark:
    palette = palette[:9]

def color_distance(c1, c2):
    return sum((a - b) ** 2 for a, b in zip(c1, c2))

def closest_palette_color(pixel):
    return min(range(len(palette)), key=lambda i: color_distance(palette[i], pixel))

pixels = img.resize((width, height)).load()
result = bytearray(width*height)
for y in range(height):
    for x in range(width):
        result[y*width+x] = closest_palette_color(pixels[x, y]) << 4
open("wallpaper.vga", "bw").write(result)

palette_img = Image.new("P", (1, 1))
palette_flat = [val for color in palette for val in color]
palette_img.putpalette(palette_flat)
img = img.resize((width, height)).quantize(palette=palette_img, dither=0).resize((width, height*2))
img.save("result.png")
