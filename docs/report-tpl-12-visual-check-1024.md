# TPL-12: 0.11.12 visual acceptance and log audit

2026-10-24, session 80. User: **"it worked. still lacks testing. check logs"**.
This accepts the preceding native-Y alignment and one-line Conform title
fixes as a short visual check. **TPL-12 functional validation remains open.**
Implementation/install: `report-tpl-12-title-alignment-1024.md`.

## Preserved evidence

Snapshot: `work/tpl12-01112/check-1024/`, 12 log files plus `audit.json`.
The snapshot preserves the game copy's custom `Logs/tgcx.log` and the 11
engine logs from the active `TGC252_Fresh082_20261019` profile. Paths,
SHA-256, byte counts and stable-read metadata are in `audit.json`; none
changed during reading. The game was no longer running at snapshot time.
The older generic Victoria II log directory was not treated as this run.

## Findings

- Custom log: **0.11.12**, own MOD-2 loader 0.8.0; reference executable
  SHA-256 matches. **206 runtime signature checks OK**, no BAD entries.
  This includes native font width/RET8/font-field and button Show/Hide.
- **No diagnostic error/fault/rejection/fallback lines** in this 250-line,
  16,728-byte custom log. In particular no Conform title-font guard warning
  and no guarded GUI read fault.
- Active profile `error.log` and `system_error.log`: **0 bytes** each.
  Setup records `interface/tgcx_army.gui` loaded. The setup/system/game
  scan found no relevant fatal/exception/failure; `stock_market_crash` is
  an ordinary static-modifier definition, not an error.
- Correct packages/settings: Army=1, Politics=0, active profile as above.
  Templates/Conform/Mobo/Replace/Gather initialize. This run cannot validate
  Bill, favours or multiplayer just because their signatures also match.
- Three templates loaded, editable; **11 live land types** resolved.
  Conform's root/tab created and activated, Conform opened for stack
  **40/415**, RUS, player-country index 65; all nine regiment registry
  round trips match. Entry log reports (172,182) from panel origin (0,150),
  consistent with current relative Y=32.
- Replace preview reads full strength **3000/3000=100%**. One probed pop
  (regiment 410) has size **350**, supports **0**, has **1**: a red-pop case.
  Another pop supports 2 and has 2: no yellow-pop overcommit shown in those
  probes. This verifies reading/probing, not a replacement being executed.
- One **"gone without its destructor; slot freed"** message is the existing
  `sweep_views()` recovery path after the view's vtable no longer matches.
  It discards the stale panel/view slot (P47); no subsequent fault is
  recorded. It does not establish long-session lifetime coverage.

## Coverage limits and next check

The log has **zero `Replace stack ...` queue summaries**, no replacement
resume summary, and no new march/merge/detach outcome. The 15 saved Conform
orders, 24 saved Gather orders and two Mobo marks are file inventory counts;
they do not demonstrate active orders resuming after a load.

Record the current result as **visual short check passed**, not full TPL-12
acceptance. Earlier 0.11.7 replacement/old-regiment detach evidence remains
in its own report; it is not a newly repeated 0.11.12 functional test.

Highest-value next test:

1. Pick an owned army with a qualifying worn regiment or red/yellow pop;
   compare strength-only/pop-only/both/neither previews and the selected
   threshold with the native panel. Check all culture modes and reopen
   persistence; no chosen template should be required.
2. Queue **Replace**. Save during construction, load that save, unpause and
   follow the new regiment through marching and merging. Confirm the old
   regiment leaves only after the merge and is detached, not disbanded.
   Include a yellow-pop overcommit and a restricted-culture/land-shortage
   case when available.
3. Preserve that run's log/screenshots and inspect for queue, resume, join
   and verified detach results. Then finish Conform/selector persistence,
   host/fleet/foreign-stack lifetime and both native-visibility transitions
   where not explicitly observed. TPL-11 rally-gather reload is separate.

Other-base and multiplayer validation remain open. No new code, build,
installation, game launch/termination, commit, push or publication here.
