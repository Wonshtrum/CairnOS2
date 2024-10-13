from sys import argv

text = open(argv[1], "r").read()

unicode_to_p437 = {
    0x2500: 196,  # ─ Single horizontal line
    0x2502: 179,  # │ Single vertical line
    0x250C: 218,  # ┌ Single upper-left corner
    0x2510: 191,  # ┐ Single upper-right corner
    0x2514: 192,  # └ Single lower-left corner
    0x2518: 217,  # ┘ Single lower-right corner
    0x251C: 195,  # ├ Single left T-junction
    0x2524: 180,  # ┤ Single right T-junction
    0x252C: 194,  # ┬ Single top T-junction
    0x2534: 193,  # ┴ Single bottom T-junction
    0x253C: 197,  # ┼ Single cross

    0x2550: 205,  # ═ Double horizontal line
    0x2551: 186,  # ║ Double vertical line
    0x2554: 201,  # ╔ Double upper-left corner
    0x2557: 187,  # ╗ Double upper-right corner
    0x255A: 200,  # ╚ Double lower-left corner
    0x255D: 188,  # ╝ Double lower-right corner
    0x2560: 204,  # ╠ Double left T-junction
    0x2563: 185,  # ╣ Double right T-junction
    0x2566: 203,  # ╦ Double top T-junction
    0x2569: 202,  # ╩ Double bottom T-junction
    0x256C: 206,  # ╬ Double cross

    0x2580: 223,  # ▀ Upper half block
    0x2584: 220,  # ▄ Lower half block
    0x2588: 219,  # █ Full block
    0x258C: 221,  # ▌ Left half block
    0x2590: 222,  # ▐ Right half block
    0x2591: 176,  # ░ Light shade
    0x2592: 177,  # ▒ Medium shade
    0x2593: 178,  # ▓ Dark shade
    0x25A0: 254,  # ■ Middle half block
}

result = bytearray()
for char in text:
    c = ord(char)
    if c < 127:
        result.append(c)
    elif c in unicode_to_p437:
        result.append(unicode_to_p437[c])
    else:
        print("Unknown character:", char)
        result.append(b'?')

open("p437.txt", "bw").write(result)
