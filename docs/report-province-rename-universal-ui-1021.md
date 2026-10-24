# Province Rename Universal 0.2.0: native UI and generic placement

2026-10-21. The user accepted the short 0.1.3 rename test, then requested
visual polish, universal compatibility like ATCU, black text, and an
icon-only entry to the right of the province name. Their final choice is
the native ABC rename sprite. **0.2.0 is built and checked, awaiting fresh
normal-exit confirmation before installation.** Installed copy remains 0.1.3.

## Accepted behavior and retained evidence

The supplied screenshot shows Mongolandia, Maxiotenia and Azhadroneland on
the map, with Azhadroneland selected and the editor open. The preserved
0.1.3 log independently records all three native command submissions and
matching name readback/header refresh, followed by reopening province 427.
This establishes the short single-player behavior. Save/reload, extended
GUI lifetime, other-base gameplay, ATCU coexistence and MP remain untested.

Unaltered screenshot and log: `work/province-rename/ui-universal-020-1021/`
as `player-013.png` and `rename-013-passed.log`. Prior crash/type reports
remain historical and are not rewritten.

## Current interface specification

- Native `GFX_bp_button_rename`, exactly 28x28, including its ABC lettering,
  background and frame. The extra Rename label is removed; the existing
  province name remains. The ABC button follows its measured text width
  with an 8px gap, constrained to the native panel's right edge.
- Editor 332x224, placed 20px beside the actual host width. On a narrow
  viewport it moves above the host when there is room, then clamps safely.
- Every added text control and the input use black native fonts. The
  title has a native parchment inset so black remains readable within the
  green header. The original base's province labels retain their own fonts.
- Genuine native/TGC green, gold braid, parchment, input frame and buttons
  are cropped/reassembled during private packaging. No drawn imitation
  braid, green texture or pencil is used.

Current board: `docs/province-rename-layout.svg`. It contains our vector
layout and external links to private artwork, with no embedded game pixels.
Exact generated preview, using the native bitmap font atlases/advances:
`work/province-rename/candidate/layout-preview-020-native.png`. The left
province panel is a placement illustration; the right editor uses the
prepared package assets. This is a preview, not a gameplay capture or
acceptance of 0.2.0. The first board is preserved verbatim at
`docs/archive/province-rename-layout-0100.svg`. The historical remote
artifact was not republished; this mechanic uses the local board.

Private art sources, resolved from the installed TGC 2.5.2 profile:
`GFX_unit_bottom_bg`, `GFX_unit_toppanel_bg`, `GFX_unit_bg_28`,
`GFX_unit_gunboat_diplo_button` and `GFX_bp_button_rename`. The resulting
five DDS files stay under ignored `work/`, like ATCU's prepared art.
This is a fixed native theme in one package; runtime does not require TGC
or read its sprites. No original base GUI/data file is bundled or replaced,
and no third-party pixels are added to tracked source (R3/R8).

## Universal runtime and evidence

The single descriptor is named **Province Rename (Universal)**. It has no
dependency, replace_path or user_dir. It adds only its own GUI definitions,
sprites, localisation and V2DLL plugin. Reference Victoria II 3.04 and
V2DLL remain requirements; the native hooks refuse a mismatched executable.
No ATCU implementation or loader was changed.

Placement follows `province_view_header` -> `province_name` using the
actual direct-child lookups (P44), full render owner chain (P37/P40), and
native host definition width. Native bitmap-font width includes proportional
glyph advances and kerning, including CP1252 accents. An unsupported font
uses the native header's right edge and logs the fallback. Missing anchors
refuse placement rather than adopting one base's guessed coordinates.

Static effective-file checks cover vanilla, TGC 2.3.4, TGC 2.5.2, the
path-corrected HPM 0.4.5 profile and DoD Fan Fork. All five retain those
native anchors and `vic_18` province text, and resolve the editor's black
fonts and sprites. Native host width is 380 in these profiles; height is
490 or 525. `base-matrix.json` records actual files and positions. This is
static compatibility evidence, not five-base gameplay acceptance. HFM and
arbitrary redesigned province panels have not been checked.

New address/ABI evidence is appended to `docs/symbols.md`; private native
exports are `window-size-asm.txt`, `text-metrics.txt`, `font-vtable.txt`,
`font-size.txt` and `font-width-asm.txt` in the evidence folder. The full
ATCU pitfall audit from the crash correction still applies: unique GUI
names, real registry/type checks, deferred release, owner coordinates,
subtree visibility/foreground and command allocation remain in place.

## Validation and exact candidate

- Reference executable SHA-256 passes; **279 guards**, including 56 Rename,
  unchanged 179 Army/runtime and 44 earlier research guards.
- Win32 Release build passes. **51 native checks**, zero failures. New
  fixtures execute the actual font width/height routines, measure kerning
  and accents, and exercise translated owners, 480px host width, unsupported
  font fallback, missing anchors and narrow viewport placement.
- **4 package checks** pass, including exact ABC sprite pixels, all-black
  fonts, unique click-transparent labels, x86 export, five-base dependency
  resolution and ZIP/DDS round trips. **4 current V2DLL loader probes**
  pass: selected/unselected, mod folder/dll_Submods, load once, unsupported
  host refuses before patching. These are not supported-game GUI tests.
- TGC scan reports anchors/range OK. Source/runtime ABI checks do not
  establish live font clipping, focus, input or click placement acceptance.

Candidate: `work/province-rename/candidate/Province-Rename-Universal-0.2.0-test-r3.zip`.
12 files (11 mod payload plus README), 250,479 bytes. SHA-256:
`5c450ba99c1f2496e936251c6c23e368faea4c5dbf73855391b5fb31f4a959da`.
DLL SHA-256: `636df479a92582952ab9c2c359c2f0d00874d805be68e0e771a1f27bd1935270`.
Earlier 0.2.0 test/r2 artifacts are retained but superseded by r3 (native ABC).
Final checks, manifest, native exports and preflight are preserved privately.

## Multiplayer assessment

**Feasible in theory, untested in MP.** Rename still constructs native
`CChangeProvinceNameCommand`, opcode 0x707, and submits it to the engine
command queue. Its native vtable has serialization, parsing and cloning
implementations; the name and province id are command payload, not a local
sidecar or direct simulation write. See the original implementation report
and symbols for serialize RVA 0x180140, parse 0x1801C0 and clone 0x1802A0.

That is the engine path intended to synchronize commands, but it is not
proof of host/client delivery, remote header refresh, checksums or OOS-free
play. A future test should use matching game/base/V2DLL/Rename builds and
settings on both peers, rename from host and client, compare names on both,
advance time, and save/reload. Mixed installations are untested. The user's
theoretical question did not authorize launching game instances.

Upstream loader reference:
<https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension>.
Upstream's MP claims about its own features do not validate this plugin.

## Installation boundary and next check

Read-only preflight of `work/Victoria-II-Province-Rename-TGC252` verifies
the exact installed 0.1.3 payload/guide and all **25,388 protected baseline
hashes**. The target executable passes all guards. No running game was
found, but the fresh normal-exit question remains unanswered. No game-copy
files were written. Main E:, D: and all other copies stay unchanged.

The exact private update script defaults to preflight only:
`work/province-rename/ui-universal-020-1021/update_020.py`. Its apply flag
may be used only after a fresh confirmation, followed by the process gate;
it backs up the full 0.1.3 payload/guide, refuses unknown conflicts, installs
only the 11 payload files, and verifies protected hashes again. No launch,
termination, commit, push, tag or publication. Next: instructions-rename-020.md.
