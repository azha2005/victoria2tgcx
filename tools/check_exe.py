#!/usr/bin/env python3
"""Executable gate: is this the v2game.exe the addresses are valid for?

Checks the SHA-256 and a list of signatures (expected bytes at RVAs
documented by Victoria2-Reverse-Engineering and V2DLL). If anything does not
match, no address in docs/ or in the DLL is valid for that executable
(AGENTS.md R2).

    python tools/check_exe.py [path to v2game.exe]

Exits with 0 if everything matches and with 1 if not.
"""

import hashlib
import sys

DEFAULT_EXE = r"E:\Victoria.II.v3.04.Inclu.ALL.DLC\v2game.exe"

# The reference executable (3.04). Measured 2026-10-12 on the D:\ and E:\n# installs, which are identical.
SHA256 = "62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648"

# .text: file offset = RVA - 0xC00 (Symbols/README.md in the RE repo).
TEXT_DELTA = 0xC00

# (RVA, expected bytes, where it comes from)
SIGNATURES = [
    (0x1D74BE, "B8E8030000", "PATCH_ALLIED_REINFORCE_150, mov eax,1000"),
    (0x282EC0, "558BEC6AFF", "FUN_00682EC0, daily sync check (OOS)"),
    (0x35BB50, "558BEC5156", "ApplyVolumes, entry"),
    (0x1D77A0, "8B81C40000003B87C4000000", "FUN_005D77A0, eligible fleet (CUnit+0xC4)"),
    (0x233826, "6A08BF0F000000684C76E000", "main menu version label"),
    # 2026-10-12, docs/symbols.md: CUpperHouseTrigger::Evaluate (reads g_countryManager)
    # and the upper house getter (CCountry+0x8B0).
    (0x4FD2A0, "558BEC83EC088B45088B15E487250156", "CUpperHouseTrigger::Evaluate"),
    (0x1E64D0, "558BEC51894DFC8B45FC05B00800008BE55DC3",
     "CCountry::GetUpperHouse, add eax,0x8B0"),
    # 2026-10-12, DLL-0: the only call to the per-frame pump FUN_009DF2B0 (rel32 = 0x5DF2B0 - 0x28572C)
    (0x285727, "E8849B3500", "call FUN_009DF2B0, DLL tick hook site"),
]


def main() -> int:
    sys.stdout.reconfigure(encoding="utf-8")  # P3: the Windows console is not UTF-8
    path = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_EXE
    with open(path, "rb") as f:
        data = f.read()

    bad = 0
    digest = hashlib.sha256(data).hexdigest()
    if digest == SHA256:
        print(f"OK   sha256 {digest[:16]}…")
    else:
        print(f"BAD  sha256 {digest} (expected {SHA256})")
        bad += 1

    for rva, expect, what in SIGNATURES:
        off = rva - TEXT_DELTA
        want = bytes.fromhex(expect)
        got = data[off:off + len(want)]
        if got == want:
            print(f"OK   RVA 0x{rva:06X}  {what}")
        else:
            print(f"BAD  RVA 0x{rva:06X}  {what}: expected {want.hex().upper()}, found {got.hex().upper()}")
            bad += 1

    print("check_exe: ALL OK" if bad == 0 else f"check_exe: {bad} failures")
    return 0 if bad == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
