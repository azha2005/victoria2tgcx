# TPL-12: native gap, visibility-based placement and 1px lift (0.11.11)

2026-10-24, session 78. User's 0.11.9 captures show excess spacing and an
empty slot when native Disband depleted is hidden. Requests: match the
native gap, move our pair when that control disappears, and **move both
up 1px**. **Final 0.11.11 installed; new behavior not tested in game.**

## Geometry and implementation

Native positions x=214/235 give **21px pitch**. Each 20px native texture
paints bounds `(0,1,19,19)`, leaving **2px between painted frames**. Our
unchanged art now uses the same pitch, in a 41x20 entry root (was 43x20).

| Native Disband depleted | Replace | Conform | Native leftmost action |
|---|---|---|---|
| Shown | (172,31) | (193,31) | Depleted (214,32) |
| Hidden | (193,31) | (214,31) | Disband (235,32) |

The Universal tick recomputes from the native control's actual shown flag
on the existing 100ms panel tick. It neither classifies the army itself
nor moves/hides native controls. Reopening, selecting another stack, and
visibility changes within the same stack follow that same placement path.
Both controls use native anchor y minus 1, as explicitly requested.

**CEU3Button shown byte is +0x4F**, not the window byte +0x67. Native Show
sets it; Hide clears it. Concrete Show/Hide slots 13/14 and the packed
position getter are checked before reading. The new Hide guard is RVA
**0x5EFEB0**, VA **0x009EFEB0**; Universal requires it and the existing
Show guard at RVA 0x5EFEF0. Hidden/missing depleted controls use ordinary
Disband; another row's control cannot anchor this row. Failed guards
disable the Universal feature safely.

Evidence: reference bytes/table RVA 0xA08BEC, plus existing read-only
export `work/archive/2026-10-19/universal-0103-button-show.txt`. An attempted
headless run could not save Java settings; no new decompilation result is
claimed. The archived export supplied the needed evidence.

Legacy injected GUI reserves the shown-depleted position; dynamic movement
is the Universal runtime path distributed here. Chooser, culture, orders,
settings formats and simulation commands are unchanged. Long army names
still have less room; native name width remains unchanged.

## Visual references (R10)

User captures preserved in `work/tpl12-01110/0119-{depleted,healthy}.png`.
Current previews, rendered by `work/tpl12-01111/render_candidate.py` and
visually inspected: `mockups/6-replace-spacing-aligned-states-1024.png`,
`6-replace-spacing-aligned-depleted-1024.png`, and
`6-replace-spacing-aligned-healthy-1024.png`.

These are actual-art composites, with header pixels under the old buttons
restored from an earlier capture; not new gameplay captures. Pre-lift
previews are retained under `6-replace-spacing-*`. Historical remote
artifact not republished. Live alignment/visibility transitions remain open.

## Validation and ZIP

- Executable: **329** checks pass; generated runtime header **203** guards;
  TGC scan anchors/range OK.
- Rust x86 offline release build, format, strict all-target Clippy,
  **23 tests** pass. Two new tests cover both visibility transitions,
  exact gaps, native offsets, missing/different-row controls and the lift.
- Python GUI **20**, Army **17**, installer/Lua **12** pass. Actual final
  Rust plugin probe passes. No C++ release rebuild.
- **50** manifest/ZIP files match; CRC OK, **415** unique GUI names. Logic
  DLLs match the build; all textures are byte-identical to 0.11.9.

ZIP: `work/tpl12-01111/ATCU-0.11.11-candidate.zip`, **1,133,925 bytes**,
with updated README and changelog.

| Item | SHA-256 |
|---|---|
| ZIP | `3450499a7a68f0b25e2a8973ab71155d08f620299bf2eeb0b3209def4dcf9895` |
| Logic DLL | `0107d7b4578e826f4ebacbb1e45df6a8f5669eb1892cd199b6a608101fe1e01f` |

## Installation and next check

Installed in `work/Victoria-II-TGC252-own-loader`, continuing the requested
fixes in the previously selected copy. Process gates found no game before
preparation or immediately before each swap; no separate normal-exit
statement claimed. **48 payload hashes pass** (46 package files, descriptor,
unchanged loader); **25,466 protected files/eight settings unchanged**.
Installed manifest updated. Evidence: `work/tpl12-01111/installation-01111.json`.

0.11.10 had completed installation when the 1px correction arrived and was
superseded without a launch. Rollbacks, each with 58 verified hashes:

- User-tested **0.11.9**: `work/tpl12-01110/before-01110/` and sidecar manifest.
- Immediate **0.11.10**: `work/tpl12-01111/before-01111/` and sidecar manifest.

Next: `instructions-tpl-12-check.md` step 4: native gap/lift, healthy and
depleted selections in both directions, same-stack changes, reopening,
both windows. Existing replacement reload/multiplayer checks remain open.
No launch, termination, main E: modification, commit, push or publication.
