"""Generates high-visibility Flat Modern 2D icons for Sedot Download Manager (SDM).
Optimized for Windows taskbar (notification area / tray) at 16, 20, 24, 32, 48, 64, 128, 256 px.
Pure 2D flat modern aesthetic, no 3D sphere/skeuomorphic effects.
"""

from pathlib import Path
import math
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "crates" / "app" / "assets" / "icons" / "app"
EXT = ROOT / "extension" / "icons"
ART = ROOT / "art"
SIZES = [16, 20, 24, 32, 48, 64, 128, 256]

# Brand Flat Colors: Modern Azure / Electric Blue
COLOR_TOP = (0, 168, 255)   # #00A8FF - Vibrant Azure
COLOR_BOT = (0, 102, 230)   # #0066E6 - Modern Royal Blue


def create_base_icon(size: int = 1024) -> Image.Image:
    """Creates a high-resolution master 2D flat modern icon for Sedot Download Manager."""
    s = size
    im = Image.new("RGBA", (s, s), (0, 0, 0, 0))

    pad = s * 0.05
    rect = [pad, pad, s - pad, s - pad]
    radius = s * 0.22

    # 1. Flat modern squircle background with clean vertical linear gradient
    for y in range(s):
        t = y / s
        r = int(COLOR_TOP[0] * (1 - t) + COLOR_BOT[0] * t)
        g = int(COLOR_TOP[1] * (1 - t) + COLOR_BOT[1] * t)
        b = int(COLOR_TOP[2] * (1 - t) + COLOR_BOT[2] * t)
        for x in range(s):
            im.putpixel((x, y), (r, g, b, 255))

    mask = Image.new("L", (s, s), 0)
    mask_draw = ImageDraw.Draw(mask)
    mask_draw.rounded_rectangle(rect, radius=int(radius), fill=255)
    im.putalpha(mask)

    draw = ImageDraw.Draw(im)
    center = s / 2

    # 2. Outer subtle rim for crisp separation on any background
    draw.rounded_rectangle(rect, radius=int(radius), outline=(255, 255, 255, 160), width=max(1, int(s * 0.012)))

    # 3. Flat 2D Suction intake chevrons (Sedot suction waves)
    w1 = s * 0.36
    y1 = s * 0.20
    h1 = s * 0.06
    th1 = int(s * 0.055)
    draw.line([(center - w1 / 2, y1), (center, y1 + h1), (center + w1 / 2, y1)], fill=(255, 255, 255, 220), width=th1, joint="curve")

    w2 = s * 0.50
    y2 = s * 0.30
    h2 = s * 0.08
    th2 = int(s * 0.065)
    draw.line([(center - w2 / 2, y2), (center, y2 + h2), (center + w2 / 2, y2)], fill=(255, 255, 255, 255), width=th2, joint="curve")

    # 4. Bold downward download arrow
    stem_w = s * 0.16
    stem_top = s * 0.42
    head_top = s * 0.58
    arrow_tip = s * 0.77
    head_w = s * 0.52

    draw.rounded_rectangle(
        [center - stem_w / 2, stem_top, center + stem_w / 2, head_top + s * 0.03],
        radius=int(stem_w * 0.25),
        fill=(255, 255, 255, 255),
    )
    draw.polygon(
        [
            (center - head_w / 2, head_top),
            (center, arrow_tip),
            (center + head_w / 2, head_top),
        ],
        fill=(255, 255, 255, 255),
    )

    # 5. Bottom suction cradle / tray platform
    bar_y = s * 0.83
    bar_w = s * 0.54
    bar_h = s * 0.06
    draw.rounded_rectangle(
        [center - bar_w / 2, bar_y, center + bar_w / 2, bar_y + bar_h],
        radius=int(bar_h / 2),
        fill=(255, 255, 255, 240),
    )

    return im


def create_tray_small_icon(size: int) -> Image.Image:
    """Specialized razor-sharp 2D flat icon for small sizes (16, 20, 24, 32)."""
    scale = 4
    s = size * scale
    im = Image.new("RGBA", (s, s), (0, 0, 0, 0))

    pad = 1.0 * scale
    rect = [pad, pad, s - pad, s - pad]
    radius = s * 0.24

    for y in range(s):
        t = y / s
        r = int(COLOR_TOP[0] * (1 - t) + COLOR_BOT[0] * t)
        g = int(COLOR_TOP[1] * (1 - t) + COLOR_BOT[1] * t)
        b = int(COLOR_TOP[2] * (1 - t) + COLOR_BOT[2] * t)
        for x in range(s):
            im.putpixel((x, y), (r, g, b, 255))

    mask = Image.new("L", (s, s), 0)
    mask_draw = ImageDraw.Draw(mask)
    mask_draw.rounded_rectangle(rect, radius=int(radius), fill=255)
    im.putalpha(mask)

    draw = ImageDraw.Draw(im)
    center = s / 2

    # Outer border for contrast on light and dark taskbars
    draw.rounded_rectangle(rect, radius=int(radius), outline=(255, 255, 255, 200), width=max(1, int(1.2 * scale)))

    # Crisp bold downward arrow
    arrow_color = (255, 255, 255, 255)

    stem_w = max(2, int(s * 0.22))
    stem_top = int(s * 0.20)
    head_top = int(s * 0.48)
    arrow_tip = int(s * 0.73)
    head_w = int(s * 0.58)

    draw.rectangle(
        [center - stem_w // 2, stem_top, center + stem_w // 2, head_top],
        fill=arrow_color,
    )
    draw.polygon(
        [
            (center - head_w // 2, head_top),
            (center, arrow_tip),
            (center + head_w // 2, head_top),
        ],
        fill=arrow_color,
    )

    bar_y = int(s * 0.79)
    bar_h = max(2, int(s * 0.10))
    bar_w = int(s * 0.58)
    draw.rectangle(
        [center - bar_w // 2, bar_y, center + bar_w // 2, bar_y + bar_h],
        fill=arrow_color,
    )

    return im.resize((size, size), Image.LANCZOS)


def main():
    APP.mkdir(parents=True, exist_ok=True)
    EXT.mkdir(parents=True, exist_ok=True)
    ART.mkdir(parents=True, exist_ok=True)

    print("Generating master 1024x1024 flat 2D icon...")
    master = create_base_icon(1024)
    master.save(ART / "Sedot Download Manager.png")

    icons = {}
    for size in SIZES:
        if size <= 32:
            icon = create_tray_small_icon(size)
        else:
            icon = master.resize((size, size), Image.LANCZOS)

        icons[size] = icon
        icon.save(APP / f"sdm-{size}.png")
        print(f"Saved {APP}/sdm-{size}.png")

    # Build sdm.ico
    big = icons[256]
    big.save(
        APP / "sdm.ico",
        sizes=[(s, s) for s in SIZES],
        append_images=[icons[s] for s in SIZES if s != 256],
    )
    print(f"Saved {APP}/sdm.ico")

    # Extension icons
    for size in (16, 32, 48, 128):
        icons[size].save(EXT / f"icon-{size}.png")
        print(f"Saved {EXT}/icon-{size}.png")

    print("Successfully built all SDM flat modern 2D icons!")


if __name__ == "__main__":
    main()
