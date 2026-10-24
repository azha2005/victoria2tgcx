# RUST-CHECK: verify the merged Rust DLL when selected

2026-10-22. PR #2 is merged and pulled; C++ remains the release default.
This card prepares a separate candidate and closes remaining Rust runtime
coverage. It does not authorize an installation or game launch by itself.

1. Read NEXT and report-pr-2-review-1022.md. Preserve unpublished Rename
   files, the existing backup ZIP and Git stash. Verify the selected commit.
2. Run `python tools/check_exe.py`. Generate headers with gen_signatures.py,
   tgcx_bill_gui.py --header, tgcx_templates_gui.py --header and
   tgcx_conform_gui.py --header. From each Rust crate's own directory build
   release/i686, check fmt and strict Clippy. Do not silently change targets.
3. Build a separate Army/Politics candidate with matching 0.11.2 GUI/data
   and the Rust DLL/loader. Keep accepted C++ packages and deployed files.
   Check actual exports, plugin entry, unsupported-exe gate and package
   compatibility. No need to copy third-party game material into git.
4. Only after an explicit install/test instruction and fresh normal-exit
   confirmation, verify no game process remains and install reversibly
   into a dedicated test copy. The user launches and exits the game.
5. Check Templates recruitment with culture/continent restrictions and
   shortage counts. Check Conform extras, missing builds, safe marching,
   merge and saved-order resumption. Compare native queued commands and
   actual outcomes, not only logs showing hooks installed.
6. Check active Bill draw/purchases/promises/count, reform eligibility,
   favour accrual and native hover/order. Capture logs/screenshots and
   preserve exact package hashes. Record untested cases explicitly; MP
   requires separate selected work.
7. Report defects with the matching C++ behavior and reproduce before
   changing shared logic. Do not switch build/release defaults based only
   on startup smoke. User decisions and TPL-8 hold still apply.
8. Close the session using ROADMAP section 7 and update next-card steps.
