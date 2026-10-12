# Our own symbols in `v2game.exe`

> **Reference.** Everything this project names or discovers in the
> executable, with its evidence (rule V6). What is already in
> `refs\Victoria2-Reverse-Engineering\Symbols\*.csv` is not repeated here:
> it is imported into the Ghidra project (`ghidra/ImportV2Symbols.java`).
>
> `conf`: **C** read in the code (bytes or decompiled), **I** inferred,
> **R** confirmed at run time. Every address used by our own code also has
> its signature in `tools/check_exe.py`.

## Functions

| VA | name | conf | evidence | date |
|---|---|---|---|---|
| `0x008FD2A0` | `CUpperHouseTrigger::Evaluate` | C | slot 6 of vtable `0x00E32768` (`docs/rtti-classes.md`); bytes disassembled by hand, below | 2026-10-12 |
| `0x008FD780` | `CUpperHouseTrigger::ParseToken` | C | slot 4; token `0x4C6` → ideology at `+0x28` (`FUN_00973BD0` by name), token `0x344` → value at `+0x20/+0x24` | 2026-10-12 |
| `0x008FD300` | `CUpperHouseTrigger::Tooltip` | I | slot 10; shows `value × 0x320000 >> 15` (= ×100, a percentage) | 2026-10-12 |
| `0x005E64D0` | `CCountry::GetUpperHouse` | C | `push ebp; mov ebp,esp; push ecx; mov [ebp-4],ecx; mov eax,[ebp-4]; add eax,0x8B0; …; ret` | 2026-10-12 |

## Structures

| structure | offset | field | type | conf | evidence |
|---|---|---|---|---|---|
| `CCountry` | `+0x8B0` | `upperHouse` (embedded `CUpperHouse`) | object | C (offset) / I (that it is the `CUpperHouse` object, vtable `0x00DFD7DC`) | `GetUpperHouse` |
| `CUpperHouse` | `+0x10` | pointer to `int64[]`, one entry per ideology | `int64*` | C | `Evaluate`: `mov eax,[eax+0x10]; mov edx,[eax+ecx*8+4]` |
| `CUpperHouse` | `[+0x10][i]` | ideology `i`'s share of the chamber, 2¹⁵ fixed point (1.0 = 32 768) | `int64` | I | the trigger compares it against the script's `value = 0.x`; the tooltip multiplies it by 100 |
| `CIdeology` | `+0x28` | the ideology's index | `int` | C | `Evaluate`: `mov ecx,[edi+0x28]` as the array index |
| (trigger scope) | `+0x0C` | the scope country's index | `int` | C | `Evaluate`: `mov eax,[eax+0xC]` → `countries[eax]` |

## How `CUpperHouseTrigger::Evaluate` was read

```
558BEC 83EC08        push ebp; mov ebp,esp; sub esp,8
8B4508               mov eax,[ebp+8]           ; scope
8B15 E4872501        mov edx,[0x012587E4]      ; g_countryManager
56 8BF1              push esi; mov esi,ecx     ; this = trigger
8B4808 8B400C        mov ecx,[eax+8]; mov eax,[eax+0xC]   ; country index
894DF8 8B4A04        mov [ebp-8],ecx; mov ecx,[edx+4]     ; CCountry*[]
8B0C81               mov ecx,[ecx+eax*4]       ; CCountry*
57 8B7E28            push edi; mov edi,[esi+0x28]         ; CIdeology*
E8 …                 call 0x005E64D0           ; eax = country + 0x8B0
8B4F28               mov ecx,[edi+0x28]        ; ideology index
8B4010               mov eax,[eax+0x10]        ; int64[]
8B54C804 3B5624      mov edx,[eax+ecx*8+4]; cmp edx,[esi+0x24]   ; high part
…                    (low part against [esi+0x20]) → true if share ≥ value
```

**Missing (GH-2):** confirm it at run time (that a country's shares sum to
32 768 and match the pie chart in the politics window) and find how many
entries the array has (all the ideologies in `ideologies.txt`, in the
file's order?).
