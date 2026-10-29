# VOL-0: agent-operated test surface assessment

2026-10-27. **The campaign test remains unrun.** No independently controlled
Windows display and input surface was established. The supported Windows
computer-use API targets host windows and its input actions activate the target;
the user has expressly excluded host desktop input and foreground game launch.
An invisible host window would not establish isolation. No launch or input was
attempted in this assessment.

## Independent checks

- `python tools/check_exe.py` returned `check_exe: ALL OK`. The disposable
  copy's `v2game.exe` SHA-256 is
  `62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648`.
- The checked ZIP SHA-256 is
  `75c1373854b0ff2200fa58375d41b78b09a175514f1f494631814331b2ec046d`;
  the staged plugin SHA-256 is
  `0400d766869a43e61fced231a83dcad959ef1b8b4eaacbaa86004c92a07b26ea`.
- The disposable descriptors identify `TGC v2.5.2 (Volunteers test)`, profile
  `TGC252_Volunteers_20261027`, and `Volunteers - technical preview`. The
  staged configuration says `mode=observe`. This checks files, not launcher
  selection or actual loader initialization.
- A read-only process snapshot found no `v2game`, `victoria2`, QEMU or VM
  console process; VMware background services were present. `Get-VM` was not
  available in the shell. The earlier bounded guest-image search is described
  in `report-volunteers-background-1027.md`; it does not prove no usable guest
  exists elsewhere. No unrelated VM was launched or modified.

Evidence files are in ignored `work/volunteers-0001/agent-test-1027/`:
`check-exe.txt`, `hashes.txt`, `configuration.txt` and `process-snapshot.txt`.
No game logs or screenshots were produced. The prior native fixture and package
test results remain in the foundation report; they do not establish gameplay or
upstream-loader acceptance.

## Outstanding acceptance

On a prepared independent surface, follow `instructions-volunteers-isolated.md`
and `instructions-volunteers-check.md`: launch the disposable copy, select the
two intended mods, observe three days with an owned army selected, change army,
save, reload, resume and exit normally. Preserve complete observer and game
logs and screenshots. Check initialization, date delta, thread/depth and army
fields as specified, without forcing refused gates. Coexistence with ATCU/PRU
is a later optional check after a base pass. Multiplayer, troop sending,
control and recall are not implemented or established by this assessment.
