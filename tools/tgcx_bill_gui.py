"""Own Bill layout and anchor-based installation; no TGC GUI enters git."""
import argparse
import json
from pathlib import Path
import re

from PIL import Image, ImageDraw
from gen_law_catalog import catalog_data
from gen_laws import BLOC, Gen, ROOT
from tgc_scan import parse

WIDTH, HEIGHT = 832, 680
LAYOUT = "tgcx_bill_layout.txt"


def inject(base, fragment):
    """Insert our child window into the one named native Politics window.

    Token spans preserve every original byte, including comments/whitespace.
    Match the window's immediate name, not a similarly named nested object.
    """
    tokens = list(re.finditer(r'#[^\r\n]*|"[^"\r\n]*"|[{}=]|[^\s{}=#"]+', base))
    tokens = [t for t in tokens if not t[0].startswith("#")]
    candidates = []
    for n in range(len(tokens)-3):
        if [t[0] for t in tokens[n:n+3]] != ["windowType", "=", "{"]:
            continue
        depth, name, end = 1, None, None
        k = n+3
        while k < len(tokens):
            t = tokens[k][0]
            if depth == 1 and t == "name" and tokens[k+1][0] == "=":
                name = tokens[k+2][0].strip('"')
            if t == "{": depth += 1
            elif t == "}": depth -= 1
            if depth == 0:
                end = tokens[k].start()
                break
            k += 1
        if name == "country_politics" and end is not None:
            candidates.append(end)
    if len(candidates) != 1 or "tgcx_bill_window" in base:
        raise ValueError("require exactly one unmodified country_politics window")
    at = candidates[0]
    result = base[:at] + "\n# >>> tgcx_bill\n" + fragment + "\n# <<< tgcx_bill\n" + base[at:]
    if not parse(result):
        raise ValueError("generated GUI is empty")
    return result


def layout():
    loc = {}
    numbers = dict(parse((Path(ROOT) / "data/tgcx_laws.txt").read_text(encoding="cp1252")))
    price = {kind: dict(values) for kind, values in numbers["price"]}
    price_text = (f"Per 5 seats: undecided Aye {price['vote']['z']}, opposed abstain {price['abstain']['m1']} / "
                  f"Aye {price['vote']['m1']}, strong abstain {price['abstain']['m2']} or promise. "
                  f"Broken debt: x{numbers['broken_debt_price_factor']}.")

    def key(name, text):
        if ";" in text:
            raise ValueError("localisation text cannot contain the CSV delimiter")
        loc[name] = text
        return name

    def text(name, label, x, y, w, h=24, font="vic_18_black", fmt="left"):
        return f'''instantTextBoxType = {{ name = "{name}" position = {{ x={x} y={y} }}
            font = "{font}" text = "{key(name, label)}" maxWidth={w} maxHeight={h} format={fmt} }}'''

    def button(name, label, x, y):
        return f'''guiButtonType = {{ name="{name}" position={{ x={x} y={y} }}
            quadTextureSprite="GFX_button_212wide" buttonText="{key(name, label)}"
            buttonFont="vic_18_black" clicksound=click }}'''

    parts = [button("tgcx_bill_open", "Open Bill", 782, -3),
             # Offscreen without the DLL: script-only Politics remains usable.
             'windowType = { name="tgcx_bill_window" position={ x=30000 y=30000 }',
             f'size={{ x={WIDTH} y={HEIGHT} }} moveable=0 fullScreen=no orientation="CENTER"',
             'backGround="" dontRender="" horizontalBorder="0" verticalBorder=""',
             'iconType={ name="tgcx_bill_bg" spriteType="tgcx_bill_bg" position={x=0 y=0} orientation="UPPER_LEFT" }',
             text("tgcx_bill_title", "Bill", 662, 8, 140, 40, "vic_title", "right"),
             text("tgcx_bill_law", "Active bill", 24, 66, 290),
             text("tgcx_bill_transition", "Count to load the bill", 24, 96, 290, 40, "Arial12_black"),
             text("tgcx_bill_vote_header", "Projected Vote", 24, 142, 290, 24, "vic_18", "centre"),
             'iconType={ name="tgcx_bill_chart_overlay" spriteType="GFX_politics_piebig_overlay" position={x=106 y=180} orientation="UPPER_LEFT" }',
             text("tgcx_bill_tallies", "Count the votes", 24, 322, 290, 24, "vic_18_black", "centre"),
             text("tgcx_bill_source", "TGC starting votes + negotiated seats", 24, 353, 290, 28, "Arial12_black", "centre"),
             text("tgcx_bill_favour_header", "Political Favours", 24, 395, 290, 24, "vic_18", "centre"),
             text("tgcx_bill_favours", "0 / 100", 30, 431, 140),
             text("tgcx_bill_rate", "+0 per month", 174, 431, 140, 24, "Arial12_black"),
             text("tgcx_bill_spent", "Spent on this bill: 0", 30, 468, 280, 24, "Arial12_black"),
             text("tgcx_bill_price_header", "Price in Favours", 24, 505, 290, 24, "vic_18", "centre"),
             text("tgcx_bill_prices", price_text,
                  30, 540, 280, 62, "Arial12_black"),
             text("tgcx_bill_bloc_header", "Upper House Blocs", 346, 67, 460, 24, "vic_18", "centre")]
    for row in range(9):
        y = 111+43*row
        parts += [text(f"tgcx_bill_bloc_{row}", "", 348, y, 154, 24, "Arial12_black"),
                  text(f"tgcx_bill_affinity_{row}", "", 506, y, 115, 24, "Arial12_black"),
                  text(f"tgcx_bill_votes_{row}", "", 624, y, 182, 24, "Arial12_black"),
                  text(f"tgcx_bill_native_{row}", "", 348, y+19, 458, 22, "Arial12_black")]
    parts += [text("tgcx_bill_crown_header", "The Crown", 346, 506, 460, 24, "vic_18", "centre"),
              text("tgcx_bill_crown", "Audience not requested", 350, 543, 452, 24, "Arial12_black"),
              text("tgcx_bill_summary", "No active bill", 346, 578, 460, 24, "vic_18_black", "centre"),
              text("tgcx_bill_hint", "Negotiate, collect favours and vote through Decisions.",
                   346, 609, 460, 22, "Arial12_black", "centre"),
              button("tgcx_bill_count", "Count the votes", 348, 643),
              button("tgcx_bill_close", "Close Bill", 580, 643), "}"]
    return "\n".join(parts)+"\n", loc


def background(path):
    # Original programmatic panel art: no copied TGC textures or mockup pixels.
    im = Image.new("RGBA", (WIDTH, HEIGHT), (43, 45, 35, 255))
    d = ImageDraw.Draw(im)
    for inset, color in ((0, "#343126"), (2, "#c1ac68"), (4, "#655b39"), (6, "#dcc580"), (9, "#898674")):
        d.rectangle((inset, inset, WIDTH-1-inset, HEIGHT-1-inset), outline=color, width=2)
    d.rectangle((13, 52, 326, 629), fill="#ccc9be", outline="#655b39", width=2)
    d.rectangle((337, 52, 818, 629), fill="#ccc9be", outline="#655b39", width=2)
    for x, y, w, label in ((20, 60, 300, 0), (20, 138, 300, 1), (20, 390, 300, 1),
                           (20, 500, 300, 1), (343, 60, 469, 0), (343, 501, 469, 0)):
        d.rectangle((x, y, x+w, y+29), fill="#31533b" if label == 0 else "#958b7c", outline="#5d594a")
        d.line((x+2, y+2, x+w-2, y+2), fill="#aba993")
    for row in range(9):
        y=106+43*row
        d.rectangle((345, y, 809, y+41), fill="#c6c3b8" if row%2 else "#d1cec2")
        d.line((345, y+41, 809, y+41), fill="#9f9b8c")
    im.save(path)


def catalogue_header():
    laws, affinity, groups, ideologies, chosen, _ = catalog_data()
    aff = {i: dict(rows) for i, rows in affinity}
    bills = next(body for key, body in laws if key == "bills")
    lines = ["// Generated from our approved affinity data and installed TGC identifiers only.",
             "#pragma once", "struct tgcx_BillInfo { const char* title; const char* from; const char* to; const char* ideal[9]; };",
             "static constexpr tgcx_BillInfo tgcx_bill_catalog[] = {"]
    for group, pair in bills:
        pair = dict(pair)
        ideal = [Gen.pretty(dict(aff[i][group])["ideal"]) for i in ideologies]
        strings = [Gen.pretty(group), Gen.pretty(pair["from"]), Gen.pretty(pair["to"])]
        lines.append("    {" + ",".join(json.dumps(s) for s in strings) + ",{" +
                     ",".join(json.dumps(s) for s in ideal) + "}},")
    lines += ["};", "static constexpr const char* tgcx_bill_bloc_names[] = {"+
              ",".join(json.dumps(BLOC[i]) for i in ideologies)+"};", ""]
    return "\n".join(lines)


def generate(output):
    output = Path(output)
    if not (output / "TGCX.mod").is_file():
        raise ValueError("Bill GUI needs a complete staged TGCX source")
    fragment, loc = layout()
    (output / "TGCX" / LAYOUT).write_bytes(fragment.encode("cp1252"))
    gfx = output / "TGCX/gfx/interface/tgcx_bill_bg.dds"
    gfx.parent.mkdir(parents=True, exist_ok=True)
    background(gfx)
    target = output / "TGCX/interface/tgcx_bill.gfx"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text('spriteTypes={ spriteType={ name="tgcx_bill_bg" texturefile="gfx/interface/tgcx_bill_bg.dds" noOfFrames=1 loadType="INGAME" } }\n', encoding="cp1252")
    rows = ["CODE;ENGLISH;FRENCH;GERMAN;POLISH;SPANISH;ITALIAN;SWEDISH;CZECH;HUNGARIAN;DUTCH;PORTUGUESE;RUSSIAN;FINNISH;x"]
    rows += [key+";"+value+";"*13+"x" for key, value in loc.items()]
    (output / "TGCX/localisation/tgcx_bill.csv").write_bytes(("\n".join(rows)+"\n").encode("cp1252"))


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--stage", type=Path)
    ap.add_argument("--header", action="store_true")
    args = ap.parse_args()
    if args.header:
        (Path(ROOT) / "dll/tgcx/tgcx_bill_catalog.h").write_text(catalogue_header(), encoding="utf-8")
    if args.stage:
        stage = args.stage.resolve()
        repo = Path(ROOT).resolve()
        game_path = any((p / "v2game.exe").is_file() for p in (stage, *stage.parents))
        if stage == repo or stage.is_relative_to(repo) or game_path or "TGC_v2.5.2" in stage.parts:
            ap.error("use private staging outside the repository, game and TGC")
        generate(stage)
    if not args.header and not args.stage:
        ap.error("use --header or --stage")


if __name__ == "__main__":
    main()
