# VOL-INSTALL-E: isolated 0.0.4 test copy

2026-10-28. The user authorizes installing the current candidate into a copy
with V2DLL, preferably E:. This supersedes the earlier no-install instruction
for this isolated target. No main-install change or game launch is implied.

1. Current intended target is E:\Victoria-II-Volunteers-0004. See the dated
   install report and its manifest for completion, hashes and verified counts.
   Do not use an incomplete target without a successful tgcx_install_manifest.json.
2. The copy contains canonical mod/TGC, a separate generated test descriptor,
   V2DLL5.98 from the prior exercised copy and Volunteers0.0.4 audit-r2.
   Keep the source/main installations unchanged. No PRU/ATCU plugin is required.
3. Run tgcx_start_preview.cmd in the test folder when testing is selected. It
   sets mode=preview and launches this copy's v2game.exe with its working
   directory and both explicit mod arguments. tgcx_start_observe.cmd changes
   only to observation mode. Both refuse to launch while another v2game runs.
   Do not change the package marker or force a failed native signature/type gate.
4. Use a new disposable SP campaign. The profile is
   TGC252_Volunteers_0004_20261028 under the usual Victoria II Documents folder;
   leave the user's normal saves/settings untouched. Start with paused UI,
   Diplomacy open/close/reopen, country/army selection and three-day observer
   samples; preserve logs. Send and Recall remain blocked in both modes.
5. Respect the source-audit limitations. Exact x86 wrapper/SEH/rollback and
   native construction/lifetime checks remain unrun. An installation hash pass
   is not gameplay acceptance. No multiplayer or functioning loan claim.
6. Later agent testing uses the existing authorized Terra/high path, computer-use
   skill and fresh observations. No blind input. Built-in F11 capture is the
   established fallback. No new agent or game is launched during this install.
7. Inspect Logs/tgcx_volunteers.log and V2DLL/Logs/ after the test; capture any
   refusal/crash and exit normally. P48: V2DLL may generate defines_v2dll.txt in
   the first mod's common folder; this target is disposable and the main/source
   installation must still be compared separately. Do not silently delete it.
8. Keep old packages/test copies and generated install evidence. No release,
   commit/push or memory update. Update current status with actual test evidence.
