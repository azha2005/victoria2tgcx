# VOL-0-ISOLATED: agent-operated game check without desktop takeover

Superseded later on 2026-10-27: the human now authorizes desktop control if
necessary using the same subagent. Follow instructions-volunteers-desktop.md.
The earlier isolation-only procedure below records the previous constraint.

2026-10-27. The user permits the agent to run its own volunteer test, while
requiring the host desktop, focus, keyboard and mouse to remain available.

1. Read report-volunteers-background-1027.md and NEXT.md. Use the existing
   disposable work/Victoria-II-Volunteers-0001 copy; do not recreate it or
   alter the main installation. Check for existing game processes read-only.
2. Establish an actually separate supported interactive surface before any
   game launch/input, for example a prepared guest session with independent
   input/display. A hidden launcher on the shared desktop does not establish
   isolation. The present computer-use surface shares the host desktop.
3. Do not start/change the user's unrelated VM or install an OS/driver as an
   implicit shortcut. If no suitable surface is available, retain this card
   as untested and continue independent background VOL-1 work. Do not send
   host input or ask the user to perform the check as the default workaround.
4. Within the separate surface, verify the reference EXE SHA, checked package
   hashes, per-active-mod loader and disposable profile. Follow the observation
   sequence in instructions-volunteers-check.md, with the agent operating it.
5. Capture the whole volunteer/game logs and separate-surface screenshots.
   Verify initialization, selected-army identity, three advancing days, pause,
   selection changes, menu/load and normal exit. Keep the original test saves.
6. Report exactly which steps ran. Advance VOL-0-CHECK only with game evidence;
   native fixtures or Ghidra exports do not count as a successful campaign.
   Coexistence and gameplay sending/control/return remain separate checks.
