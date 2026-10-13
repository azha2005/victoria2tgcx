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
from tgcx_pe import PE

DEFAULT_EXE = r"E:\Victoria.II.v3.04.Inclu.ALL.DLC\v2game.exe"

# The reference executable (3.04). Measured 2026-10-12 on the D:\ and E:\n# installs, which are identical.
SHA256 = "62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648"

# .text: file offset = RVA - 0xC00 (Symbols/README.md in the RE repo).
TEXT_DELTA = 0xC00

# (RVA, expected bytes, where it comes from)
SIGNATURES = [
    (0xA0E48C, "C0D06000A0FF6D00007D7300504472003034750090E17900", "LAW-2 CButtonObserverGlue<CPoliticsView> table"),
    (0x5E4C80, "558BEC568BF1578B7D088D866C020000508D8E7C02000051", "LAW-2 native button lookup"),
    (0x5CD610, "8BC18B500885D274088B480C034804FFE2C3CCCCCCCCCCCC", "LAW-2 button observer native trampoline"),
    (0xA0A58C, "C0D06000401C640010D69C0050D27B00A0BC9400C0BC9400", "LAW-2 CButtonObserverGlue primary vtable"),
    (0x9FBB60, "F0D64700A01B6400106B9E00A01B7E00B01B7E0050C64300", "LAW-2 CVariables primary vtable"),
    (0x241BA0, "558BEC8B45088378141072028B008B11508B4208FFD05DC2", "LAW-2 CVariables Find(std::string)"),
    (0x5E4A20, "558BEC83E4F864A1000000006AFF68A646B5005064892500", "LAW-2 native child-window lookup"),
    (0x5E4D00, "558BEC8D81CC020000508B450881C1DC02000051E8171E00", "LAW-2 native text-box lookup"),
    (0x8ED0, "558BEC538B5D0C568BF18B4D08578B79103BFB730A6818DADE00", "LAW-2 native std::string assign"),
    (0x65ECC0, "558BEC64A1000000006AFF6818D9B300508B450864892500", "LAW-2 native GUI text SetText ESI ABI"),
    (0x2DCE10, "558BEC83E4F864A1000000006AFF68A90EBA00508B450864", "LAW-2 native decision click handler"),

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
    # DLL-1/HEMI-1: scoped Ghidra evidence, docs/report-dll-12-1012.md.
    (0xA0E45C, "002B6D00102C6D00", "CPoliticsView destructor/show slots"),
    (0xA0E468, "402C6D00", "CPoliticsView hide slot"),
    (0xA0E480, "D02D6D00", "CPoliticsView tooltip slot"),
    (0x4FD801, "A13C8A2501E8C5630700", "ideology manager lookup operand"),
    (0x5793C0, "558BEC83E4F8518B41142B4110535633", "CUpperHouse vector begin/end"),
    (0x5E3BA0, "558BEC83EC0C8B9190000000", "GUI centred position setter"),
    (0x5E4450, "568BF18B8E48020000C6466701", "GUI show flag"),
    (0x5E4030, "568BF18B8E48020000C6466700", "GUI hide flag"),
    (0x59A520, "FF868CBC060080BEA0BC06000074258B", "FUN_0099A520 final engine Present entry"),
    (0xA38324, "B0599900202E9A00806B990090DC9900", "native renderer primary vtable"),
    (0xA19B8C, "D0077E00202E9A0060097E0090DC9900", "CEU3Graphics live renderer primary vtable"),
    (0x599F22, "80BEF00000000076158B43648B088B91E400", "ordered GUI render loop after entity Draw, FUN_00999D00"),
    (0xA3C11C, "404D9E00", "native root icon lookup slot, FUN_009E4D40"),
    (0xA3A930, "007EA80060E29E00B06B9A0020366300", "native icon primary vtable"),
    (0xA3FF30, "20F1A200", "native overlay entity Draw slot, FUN_00A2F120"),
    # HEMI-3: native scalar hover selection; structured tooltip slot stays untouched.
    (0x2D52C6, "E8C5B930008BCE3AC3", "Politics pie slice CALL, FUN_006D2DD0"),
    (0xA3BDA8, "700B9E00", "pie entity hit-test slot +0x50, FUN_009E0B70"),
    (0x5E0B70, "558BEC83E4F883EC08568BF18B8E68010000", "pie hit-test ECX/float arguments, x87 result, RET 32"),
    (0x5E0C90, "558BEC83E4F883EC18660F6E4508660F6E4D0C", "pie slice EAX/int arguments, AL result, RET 16"),
]


# Research-only gates, separate from the guards generated into the DLL.
# Promote a function to SIGNATURES before calling or hooking it at runtime.
RESEARCH_SIGNATURES = [
    (0x337D00, "8B410885C074058B4904FFE0C3CCCCCC558BEC8B55088948", "Politics observer click slot (LAW-2 research)"),
    (0x2DFFA0, "558BEC8B412085C074068B49045DFFE05DC20400CCCCCCCC", "Politics observer slot 1 (LAW-2 research)"),
    (0x2D26D0, "100800837DEC10720C8B45D850E839C23D0083C404C745EC", "Politics observer construction region (LAW-2 research)"),
    (0x505B30, "558BEC64A1000000006AFF68CB23B4005064892500000000", "FUN_00905B30, decision message construction (LAW-2 research)"),
    (0x241C40, "558BEC8BC18B503885D274098B483C0348045DFFE25DC204", "FUN_00641C40, button observer slot 1 (LAW-2 research)"),
    (0x5A29F0, "558BEC568B7508837E04FF578BF97438", "FUN_009A29F0, variables second table Find slot (LAW-2 research)"),
    (0x241BA0, "558BEC8B45088378141072028B008B11", "FUN_00641BA0, variables Find slot (LAW-2 research)"),
    (0x5E4A20, "558BEC83E4F864A1000000006AFF68A6", "FUN_009E4A20, root window lookup (LAW-2 research)"),
    (0x5E4D00, "558BEC8D81CC020000508B450881C1DC", "FUN_009E4D00, root text lookup (LAW-2 research)"),
    (0x474F80, "558BEC6AFF68F026B40064A100000000", "FUN_00874F80, decision execution command path (LAW-2 research)"),
    (0x8ED0, "558BEC538B5D0C568BF18B4D08578B79", "FUN_00408ED0, native string assign (LAW-2 research)"),
    (0x65ECC0, "558BEC64A1000000006AFF6818D9B300", "FUN_00A5ECC0, native text SetText (LAW-2 research)"),
    (0x4DA5C0, "558BEC8B450883EC0883780C00535657", "FUN_008DA5C0, variable trigger lookup (LAW-2 research)"),
    (0x2DCE10, "558BEC83E4F864A1000000006AFF68A90EBA", "FUN_006DCE10, native decision click handler (LAW-2 research)"),
    (0x4AE290, "558BEC8B450C3DBE030000740E508B45", "FUN_008AE290, modifier factor field parser"),
    (0x83D41C, "BA304AE2008DB564CBFFFFE8A4B1D4FF8BC6B9F3", "support token registration, add political"),
    (0x4E8AB0, "558BEC83E4F86AFF686BBCB40064A100", "FUN_008E8AB0, militancy trigger by scope"),
    (0x4FB3E0, "558BEC83E4F88B450883EC08568BF18B", "FUN_008FB3E0, average militancy trigger"),
    (0x125A20, "558BEC83E4F88B451083EC5053C70700", "FUN_00525A20, social support aggregation"),
    (0x125BE0, "558BEC83E4F88B450C83EC50538B5D10", "FUN_00525BE0, political support aggregation"),
    (0x126A30, "558BEC6AFF680F1DBA0064A100000000", "FUN_00526A30, social reform support tooltip"),
    (0x1273A0, "558BEC6AFF680F1DBA0064A100000000", "FUN_005273A0, political reform support tooltip"),
    (0x58FC80, "558BEC83EC08538B5D08568D45FC5033", "FUN_0098FC80, decimal-to-thousandths parser"),
    (0x572260, "558BEC64A1000000006AFF688883B500", "FUN_00972260, ideology construction/support objects"),
    (0x5A1CE0, "558BEC51568BF08B4E1C8B018B5008FF", "FUN_009A1CE0, additive numeric field parsing"),
    (0x5A0F40, "558BEC6AFF68C1A1B40064A100000000", "FUN_009A0F40, additive parser fallback"),
    (0x4AF810, "558BEC83E4F883EC1C8B411C8B550889", "FUN_008AF810, additive upper-house support evaluator"),
    (0x4AEC00, "558BEC83E4F883EC1C8B51242B51208B", "FUN_008AEC00, general additive evaluator comparison"),
    (0x4AF210, "558BEC64A1000000006AFF6868C1B500", "FUN_008AF210, additive support parser"),
    (0x572B20, "558BEC518B450C5356578BF93DE70200", "FUN_00972B20, ideology parser"),
    (0x4FE3C0, "558BEC83EC088B45088B50088B400C8B", "FUN_008FE3C0, country social movement trigger"),
    (0x4FE6E0, "558BEC83EC088B45088B50088B400C8B", "FUN_008FE6E0, country political movement trigger"),
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

    image = PE(path)
    for rva, expect, what in SIGNATURES + RESEARCH_SIGNATURES:
        off = image.offset(rva)
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
