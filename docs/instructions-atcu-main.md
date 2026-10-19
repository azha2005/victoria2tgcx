# ATCU main-install verification and rollback

2026-10-19. Already installed ATCU0.0.1 in the main E: game after fresh
normal-exit confirmation. Do not reinstall. Read report-atcu-main-install-1019.md.

1. User opens E:/Victoria.II.v3.04.Inclu.ALL.DLC/victoria2.exe normally.
   Select ATCU - Army Template and Conform Universal plus base; for vanilla,
   ATCU alone. Untick old Army and old TGCX Politics so its older DLL cannot
   take loader precedence. Never launch/exit for the user.
2. Check Templates/recruit and Conform during normal gameplay. Log should
   identify internal0.10.4 through existing MOD-2 loader0.8.0. Culture and
   continent filters are still research, not part of this release.
3. Preserve main Logs/tgcx.log after normal exit if a defect is reported.
   Check exact loaded version and selections before changing offsets.
4. Rollback is selecting the previous Army/Politics combination with ATCU
   unticked. All old files/loader remain. Backups and full hashes are in
   work/main-atcu-001-1019/. No base file or template preference was changed.
5. Additional installs require fresh normal-exit confirmation/process gate.
   No new publication, commit/push or feature work follows implicitly.
