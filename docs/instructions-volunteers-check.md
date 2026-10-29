# VOL-0: observe-only campaign check

2026-10-27. Independent Rust build 0.0.1. This checks the probe before VOL-1;
it is not a test of sending/commanding volunteers because that is not enabled.

Latest user instruction now authorizes desktop control if necessary, using the
same Terra/high subagent. First follow instructions-volunteers-desktop.md.
The manual launch/exit wording below is retained as the original campaign
recipe; the agent should operate its own disposable test under that permission.

1. Read `docs/volunteers-design.md`. Use the staged technical-preview ZIP
   recorded in `docs/report-volunteers-foundation-1027.md`.
2. Exit the game normally yourself before installing. Use an isolated copy,
   retain a backup of its mod folder/settings, and copy the ZIP's mod folder
   into that copy. Do not replace the loader, ATCU or PRU DLLs.
3. Use V2DLL's per-active-mod plugin loading as for PRU. The plugin filename
   is mod/tgcx_volunteers/v2dll_mod.dll. Select Volunteers - technical preview
   with your base. Keep tgcx_volunteers.cfg mode=observe.
4. Launch the game yourself. Start/load a disposable SP campaign. Pause and
   select an army you own. There is no new volunteer button in this build.
5. Advance at least three days with that army selected. Check
   Logs/tgcx_volunteers.log for observer_installed=true and samples with
   date delta Some(24), stable thread ID, depth_max=1 and expected army IDs,
   province, owner, affiliation_plus_CC and first_pop_owner.
6. Pause, change army, resume. Save, return to menu, load that save, resume.
   Preserve the whole log. A world/date transition or sample_unavailable line
   is evidence to inspect, not an automatic game failure; no writes occur.
7. Exit normally yourself. Review the log and game errors before enabling
   any later command prototype. If initialization refuses, inspect the exact
   hash/signature/slot/config line; do not force its gate open.
8. Only after the isolated base check, repeat with ATCU and PRU ticked, keeping
   all existing files. Compare startup and native features. This new plugin's
   coexistence is not established by their older acceptance results.
9. No MP behavior is claimed. A two-peer read-only observer comparison needs
   separately selected work; do not enable experimental volunteer commands.

Agent next steps after the log: reconcile tick/load/thread observations;
continue VOL-1 native command authorization and exact pop-affiliation handling.
Do not implement direct owner writes to make a demonstration appear to work.
