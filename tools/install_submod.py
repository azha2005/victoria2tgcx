#!/usr/bin/env python3
"""Install or remove the submod in the game's mod\\ folder.

    python tools/install_submod.py <mod folder>            # install (or reinstall)
    python tools/install_submod.py <mod folder> --remove   # remove

Copies submod\\TGCX\\ and submod\\TGCX.mod into <mod folder> (a copy, not a
link: the launcher may not follow links). Touches nothing else in mod\\ (R1).

Fixed-name files the engine reads only once (common\\event_modifiers.txt)
cannot be ours alone: they are generated here from the installed TGC v2.5.2
(D1) plus our block, submod\\tgcx_event_modifiers.txt (written by
tools/gen_laws.py). They never enter git (R3, R8).

Before deleting an existing TGCX folder it checks that the folder is ours:
every file in it must start with the tgcx_ prefix (R4), except the generated
ones. If any does not, it refuses and deletes nothing.
"""

import argparse
import os
import re
import shutil
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(ROOT, "submod")
NAME = "TGCX"
PREFIX = "tgcx_"
# generated file in TGCX -> (TGC's file it starts from, our block appended to it)
GENERATED = {
    os.path.join("common", "event_modifiers.txt"): (os.path.join("common", "event_modifiers.txt"),
                                                    os.path.join(SRC, "tgcx_event_modifiers.txt")),
}


def foreign_files(folder):
    """Files in the folder that do not carry the submod prefix. A leading sort
    prefix of digits and underscores is allowed (000_tgcx_*.csv sorts before
    TGC's 00_*.csv, which localisation needs to win: P20)."""
    bad = []
    for dirpath, _, files in os.walk(folder):
        for f in files:
            if os.path.relpath(os.path.join(dirpath, f), folder) in GENERATED:
                continue
            if not re.sub(r"^[0-9_]+(?=tgcx_)", "", f.lower()).startswith(PREFIX):
                bad.append(os.path.relpath(os.path.join(dirpath, f), folder))
    return bad


def is_our_mod_file(path):
    with open(path, "rb") as f:
        text = f.read().decode("cp1252", errors="replace")
    return f'path = "mod/{NAME}"' in text


def remove(mod_dir):
    folder = os.path.join(mod_dir, NAME)
    modfile = os.path.join(mod_dir, NAME + ".mod")
    if os.path.isdir(folder):
        bad = foreign_files(folder)
        if bad:
            sys.exit(f"install_submod: {folder} has files without {PREFIX}, leaving it alone: {bad[:5]}")
    if os.path.isfile(modfile) and not is_our_mod_file(modfile):
        sys.exit(f"install_submod: {modfile} does not point to mod/{NAME}, leaving it alone")
    if os.path.isdir(folder):
        shutil.rmtree(folder)
        print(f"removed   {folder}")
    if os.path.isfile(modfile):
        os.remove(modfile)
        print(f"removed   {modfile}")


def generate(tgc, dest):
    """TGC's file, byte for byte, then our block: the same bytes every time."""
    for rel, (tgc_rel, block) in GENERATED.items():
        base = os.path.join(tgc, tgc_rel)
        if not os.path.isfile(base):
            sys.exit(f"install_submod: {base} not found: is --tgc the TGC v2.5.2 folder?")
        if not os.path.isfile(block):
            sys.exit(f"install_submod: {block} not found: run tools/gen_laws.py first")
        with open(base, "rb") as f:
            data = f.read()
        with open(block, "rb") as f:
            ours = f.read()
        nl = b"\r\n" if b"\r\n" in data[:4096] else b"\n"
        out = os.path.join(dest, rel)
        os.makedirs(os.path.dirname(out), exist_ok=True)
        with open(out, "wb") as f:
            f.write(data.rstrip() + nl + nl + ours.replace(b"\r\n", b"\n").replace(b"\n", nl))
        print(f"generated {rel} = TGC's ({len(data)} bytes) + ours ({len(ours)} bytes)")


def install(mod_dir, tgc):
    src = os.path.join(SRC, NAME)
    bad = foreign_files(src)
    if bad:
        sys.exit(f"install_submod: the submod has files without {PREFIX} (R4): {bad}")
    remove(mod_dir)  # a reinstall leaves no stale files behind
    shutil.copytree(src, os.path.join(mod_dir, NAME))
    generate(tgc, os.path.join(mod_dir, NAME))
    shutil.copy2(os.path.join(SRC, NAME + ".mod"), os.path.join(mod_dir, NAME + ".mod"))
    n = sum(len(fs) for _, _, fs in os.walk(src))
    print(f"installed {NAME} ({n} files + {len(GENERATED)} generated) + {NAME}.mod in {mod_dir}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("mod_dir", help=r"the game's mod\ folder")
    ap.add_argument("--remove", action="store_true")
    ap.add_argument("--tgc", help=r"TGC folder the generated files start from (default: <mod>\TGC_v2.5.2\TGC)")
    a = ap.parse_args()
    if not os.path.isdir(a.mod_dir) or not any(f.lower().endswith(".mod") for f in os.listdir(a.mod_dir)):
        sys.exit(f"install_submod: {a.mod_dir} does not look like a mod\\ folder (no .mod files)")
    if a.remove:
        remove(a.mod_dir)
    else:
        install(a.mod_dir, a.tgc or os.path.join(a.mod_dir, "TGC_v2.5.2", "TGC"))


if __name__ == "__main__":
    main()
