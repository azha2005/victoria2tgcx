# VOL-LOADER: CreatineCore 6.36 startup and performance acceptance

2026-10-29. User explicitly authorized replacing the loader. Installed only in
`E:/Victoria-II-Volunteers-0004`; user retains desktop/game input. Read the report
and P78. Keep `work/volunteers-loader-636/before-636` for rollback.

1. Run the SAME `E:/Victoria-II-Volunteers-0004/tgcx_start_prototype.cmd`.
   Its script selects carrier, TGC, Volunteers in that order. Do not use the
   ordinary launcher to reconstruct the order or change the main installations.
2. Check startup log at `z_CreatineCore/Logs/v2dll.log`, not the preserved inactive
   `V2DLL/Logs` tree. Confirm version 6.36, global settings used, probes off,
   one Volunteers `v2dll_mod.dll -> ok` and the performance plugin admitted.
   Performance verbose logging and FIX_ARMY_WINDOW_LAG must be off. The optional
   personal-patches plugin should not appear. No duplicate Volunteers/core copy.
3. Check `Logs/tgcx_volunteers.log`: version 0.0.12 and all native GUI/load/journal/
   scoped hook installation flags true. A refused signature is a failed migration
   acceptance, not something to bypass. Preserve both logs on failure.
4. Verify the expected test-profile save list and map/UI. Start with a disposable
   copy of the same pre-upgrade save. No save-format or full gameplay acceptance
   is implied by the proxy/API fixture. Sending remains SP-only.
5. Confirm `mod/tgcx_loader_data/common/defines_ccore.txt` is the upstream generated
   data target and TGC's old defines file remains unchanged. A loader request to
   rename/append under TGC means the explicit carrier selection failed; stop and
   fix the launch path. Retain pre/post hashes, not assumptions about load order.
6. Compare the same 30-day range after warm-up at identical speed/camera/panel
   state, both with and without an active volunteer mission where available.
   Compare game-day wall time; user identified simulation speed as the issue.
   Use plugin_timing deltas with their sampling/precision limits. Do not attribute
   all improvement to a single option because both loader and one stutter gate
   changed together. Preserve logs and actual elapsed times.
7. Retest sending, normal donor control, movement, battle, siege start/join/progress
   and completed capture credit, recall/peace/donor-war return, loss and save/load.
   Full recipient vision and lifecycle acceptance remain open. Follow existing
   prototype/control/OOS cards; no new MP transport was introduced by the loader.
8. Roll back only after normal exit: restore every path listed as replaced in
   `before-636/rollback.json` from that backup. Move only the listed new core/
   carrier directories and carrier descriptor to a safe private backup after
   checking their resolved absolute paths remain in this isolated copy. Old V2DLL
   core/config were preserved; the restored dinput8 proxy selects them again.
   Keep state/RNG probes off from the pre-migration 0.0.12 settings.
9. Source checks: `python tools/test_creatinecore_migration.py` and
   `python tools/check_exe.py E:/Victoria-II-Volunteers-0004/v2game.exe`.
   Native fixture sources/upstream header are private in work/volunteers-loader-636;
   do not commit raw upstream material. No Rust rebuild is required unless
   Volunteers source changes. Archive NEXT, update cards and next concrete steps.
