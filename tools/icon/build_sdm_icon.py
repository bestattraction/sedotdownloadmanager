"""Generates high-visibility icons for Sedot Download Manager (SDM).
Optimized for Windows taskbar (notification area / tray) at 16, 20, 24, 32, 48, 64, 128, 256 px.
"""

from pathlib import Path
import math
from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "crates" / "app" / "assets" / "icons" / "app"
EXT = ROOT / "extension" / "icons"
ART = ROOT / "art"
SIZES = [16, 20, 24, 32, 48, 64, 128, 256]


def create_base_icon(size: int = 1024) -> Image.Image:
    """Creates a high-resolution 1024x1024 master icon for Sedot Download Manager."""
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(im)

    center = size / 2
    radius = size * 0.44

    # 1. Base vibrant gradient circle (Electric Cyan / Luminous Turquoise to Vivid Royal Blue)
    # Using radial + vertical linear gradient for rich 3D sphere/badge feel
    for y in range(size):
        for x in range(size):
            dx = x - center
            dy = y - center
            dist = math.sqrt(dx * dx + dy * dy)
            if dist <= radius:
                # Vertical factor (0 top to 1 bottom)
                vy = (y - (center - radius)) / (2 * radius)
                # Diagonal light factor
                light = 1.0 - (dist / radius) * 0.3 + (center - y) / (2 * radius) * 0.25
                light = max(0.0, min(1.3, light))

                # Gradient: Vibrant Cyan (0, 230, 255) to Deep Electric Blue (0, 95, 240)
                r = int((0 * (1 - vy) + 0 * vy) * light)
                g = int((235 * (1 - vy) + 105 * vy) * light)
                b = int((255 * (1 - vy) + 245 * vy) * light)

                # Outer border anti-aliasing
                alpha = 255
                if dist > radius - 2.5:
                    alpha = int(255 * (radius - dist) / 2.5)

                im.putpixel((x, y), (min(255, r), min(255, g), min(255, b), max(0, min(255, alpha))))

    # 2. Glowing outer rim for high visibility on dark and light taskbars
    rim_layer = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    rim_draw = ImageDraw.Draw(rim_layer)
    rim_draw.ellipse(
        [center - radius, center - radius, center + radius, center + radius],
        outline=(255, 255, 255, 180),
        width=int(size * 0.025),
    )
    im = Image.alpha_composite(im, rim_layer)

    # 3. Draw the bold "Sedot" downward suction arrow and energy rings
    # Arrow properties
    arrow_layer = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    arrow_draw = ImageDraw.Draw(arrow_layer)

    # Top suction chevron / wave
    top_y = size * 0.24
    chev_h = size * 0.08
    chev_w = size * 0.44
    chev_thickness = size * 0.085

    # Chevron 1 (Top suction intake wave)
    p1 = (center - chev_w * 0.7, top_y)
    p2 = (center, top_y + chev_h)
    p3 = (center + chev_w * 0.7, top_y)
    arrow_draw.line([p1, p2, p3], fill=(255, 255, 255, 240), width=int(chev_thickness), joint="curve")

    # Chevron 2 (Mid suction intake wave)
    mid_y = top_y + size * 0.14
    p4 = (center - chev_w * 0.85, mid_y)
    p5 = (center, mid_y + chev_h * 1.1)
    p6 = (center + chev_w * 0.85, mid_y)
    arrow_draw.line([p4, p5, p6], fill=(255, 255, 255, 245), width=int(chev_thickness * 1.05), joint="curve")

    # Main bold downward download arrow
    arrow_top = mid_y + size * 0.14
    arrow_bottom = size * 0.78
    stem_w = size * 0.16
    head_w = size * 0.52
    head_top = size * 0.56

    # Arrow stem
    arrow_draw.rounded_rectangle(
        [center - stem_w / 2, arrow_top, center + stem_w / 2, head_top + size * 0.05],
        radius=int(stem_w * 0.3),
        fill=(255, 255, 255, 255),
    )

    # Arrow head (triangle)
    head_pts = [
        (center - head_w / 2, head_top),
        (center, arrow_bottom),
        (center + head_w / 2, head_top),
    ]
    arrow_draw.polygon(head_pts, fill=(255, 255, 255, 255))

    # Bottom suction cradle line (tray/platform)
    plat_y = size * 0.84
    plat_w = size * 0.56
    plat_h = size * 0.06
    arrow_draw.rounded_rectangle(
        [center - plat_w / 2, plat_y, center + plat_w / 2, plat_y + plat_h],
        radius=int(plat_h / 2),
        fill=(255, 255, 255, 230),
    )

    # Composite with shadow for contrast
    shadow = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    shadow_draw = ImageDraw.Draw(shadow)
    # Draw blurred shadow behind white elements
    shadow.paste((0, 30, 80, 120), (0, int(size * 0.02)), arrow_layer.split()[3])
    shadow = shadow.filter(ImageFilter.GaussianBlur(radius=size * 0.02))

    im = Image.alpha_composite(im, shadow)
    im = Image.alpha_composite(im, arrow_layer)

    return im


def create_tray_small_icon(size: int) -> Image.Image:
    """Specialized high-contrast, razor-sharp icon for small tray sizes (16, 20, 24, 32)."""
    # Scale 4x for clean supersampling
    scale = 4
    s = size * scale
    im = Image.new("RGBA", (s, s), (0, 0, 0, 0))
    draw = ImageDraw.Draw(im)

    pad = 1.0 * scale
    center = s / 2

    # Bright gradient circle: Cyan to Bright Blue
    # Top color: (0, 230, 255), Bottom color: (0, 110, 255)
    r = (s - pad * 2) / 2
    for y in range(int(pad), int(s - pad)):
        for x in range(int(pad), int(s - pad)):
            dx = x - center
            dy = y - center
            dist = math.sqrt(dx * dx + dy * dy)
            if dist <= r:
                vy = (y - pad) / (2 * r)
                # Outer border anti-aliasing
                alpha = 255
                if dist > r - 1.5:
                    alpha = int(255 * (r - dist) / 1.5)
                # Electric Cyan (#00E5FF) to Radiant Blue (#0066FF)
                cr = int(0 * (1 - vy) + 0 * vy)
                cg = int(230 * (1 - vy) + 110 * vy)
                cb = int(255 * (1 - vy) + 255 * vy)
                im.putpixel((x, y), (cr, cg, cb, max(0, min(255, alpha))))

    # Bright white border for contrast against dark Windows taskbar
    draw.ellipse([pad, pad, s - pad, s - pad], outline=(255, 255, 255, 220), width=max(1, int(1.2 * scale)))

    # Crisp bold downward arrow
    arrow_color = (255, 255, 255, 255)

    stem_w = max(2, int(s * 0.22))
    stem_top = int(s * 0.22)
    head_top = int(s * 0.50)
    arrow_tip = int(s * 0.74)
    head_w = int(s * 0.58)

    # Stem
    draw.rectangle(
        [center - stem_w // 2, stem_top, center + stem_w // 2, head_top],
        fill=arrow_color,
    )
    # Head
    draw.polygon(
        [
            (center - head_w // 2, head_top),
            (center, arrow_tip),
            (center + head_w // 2, head_top),
        ],
        fill=arrow_color,
    )

    # Bottom cradle bar
    bar_y = int(s * 0.80)
    bar_h = max(2, int(s * 0.10))
    bar_w = int(s * 0.56)
    draw.rectangle(
        [center - bar_w // 2, bar_y, center + bar_w // 2, bar_y + bar_h],
        fill=arrow_color,
    )

    # Resize down to target size with Lanczos
    return im.resize((size, size), Image.LANCZOS)


def main():
    APP.mkdir(parents=True, exist_ok=True)
    EXT.mkdir(parents=True, exist_ok=True)
    ART.mkdir(parents=True, exist_ok=True)

    print("Generating master 1024x1024 icon...")
    master = create_base_icon(1024)
    master.save(ART / "Sedot Download Manager.png")

    icons = {}
    for size in SIZES:
        if size <= 32:
            # Specially optimized for small taskbar displays
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

    print("Successfully built all SDM icons!")


if __name__ == "__main__":
    main()
