"""Army art lookups against the base's effective file set (including replace_path)."""
from pathlib import Path

from PIL import Image, ImageDraw
from tgc_scan import parse, read
from tgcx_army_catalog import profile_for


def filter_box():
    """Fallback 20px checkbox frame when a base lacks the native checkbox art."""
    image = Image.new('RGBA', (20, 20), (232, 226, 204, 255))
    draw = ImageDraw.Draw(image)
    draw.rectangle((0, 0, 19, 19), outline=(67, 57, 35, 255))
    draw.rectangle((1, 1, 18, 18), outline=(159, 142, 93, 255))
    return image


def filter_checkbox(folder, profile=None):
    """Native GFX_checkbox_default frames (off, on) and the button art they share.

    The button keeps only the pixels both frames have in common, so the
    off/on art drawn beneath it shows through unchanged and the button stays
    on top for clicks."""
    try:
        strip = texture(folder, "gfx/interface/checkbox_default.dds", profile)
    except ValueError:
        strip = None
    if strip is None or strip.size != (40, 20):
        off = filter_box()
        on = off.copy()
        draw = ImageDraw.Draw(on)
        draw.line((5, 10, 8, 14), fill=(46, 38, 20, 255), width=2)
        draw.line((8, 14, 15, 5), fill=(46, 38, 20, 255), width=2)
    else:
        off, on = strip.crop((0, 0, 20, 20)), strip.crop((20, 0, 40, 20))
    shared = off.copy()
    pixels = shared.load()
    a, b = off.load(), on.load()
    for y in range(20):
        for x in range(20):
            if a[x, y] != b[x, y]:
                pixels[x, y] = (0, 0, 0, 0)
    return off, on, shared


def sprite_specs(folder, profile=None):
    found = {}

    def walk(items):
        for _, value in items:
            if not isinstance(value, list):
                continue
            fields = {k: v for k, v in value if not isinstance(v, list)}
            name = fields.get("name", "").strip('"')
            if name and "texturefile" in fields:
                found.setdefault(name, fields)
            walk(value)

    base = profile or profile_for(folder)
    # Base declarations win when vanilla has the same sprite under a
    # different filename. An overridden file is already removed by files().
    files = sorted(base.files("interface", "*.gfx").values(),
                   key=lambda p: (not (base.root and p.is_relative_to(base.root)), str(p)))
    for path in files:
        walk(parse(read(path)))
    return found


def sprites(folder, profile=None):
    return {name: fields["texturefile"].strip('"').replace("\\", "/")
            for name, fields in sprite_specs(folder, profile).items()}


def texture(folder, path, profile=None):
    base = profile or profile_for(folder)
    rel = Path(path)
    for candidate in (rel, rel.with_suffix(".dds"), rel.with_suffix(".tga")):
        found = base.find(candidate.as_posix())
        if found:
            with Image.open(found) as image:
                return image.convert("RGBA")
    raise ValueError(f"installed texture not found in effective base: {path}")


def sprite(folder, table, name, frame=0, frames=1, profile=None):
    image = texture(folder, table[name], profile)
    if frames < 1 or image.width % frames or frame < 0 or frame >= frames:
        raise ValueError(f"invalid sprite frame metadata for {name}")
    width = image.width // frames
    return image.crop((width * frame, 0, width * (frame + 1), image.height))


def unit_frames(folder, profile=None):
    specs = sprite_specs(folder, profile)
    if "GFX_unit_strip" not in specs:
        raise ValueError("base does not define GFX_unit_strip")
    frames = int(specs["GFX_unit_strip"].get("noOfFrames", "1"))
    if not 1 <= frames <= 1024:
        raise ValueError("unsupported unit-strip frame count")
    return frames
