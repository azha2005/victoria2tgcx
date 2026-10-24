# Province Rename Universal 0.2.0 installation addendum

2026-10-21. **Installed in the dedicated TGC/V2DLL copy.** This supersedes
the pending-install status in report-province-rename-universal-ui-1021.md.

The user explicitly instructed "install it" after being told installation
was awaiting exit confirmation. That fresh direction authorized this
installation; it is not recorded as a literal normal-exit confirmation.
No v2game.exe process was present, including hidden processes. The process
gate was checked before and throughout installation. No game was launched
or terminated.

Target: `work/Victoria-II-Province-Rename-TGC252`. Exact final r3 candidate
installed, including native 28x28 ABC entry right of the province name,
black editor text, native green/braids/parchment and generic name anchoring.

- All **11 mod payload files** match the candidate manifest after copying.
- All **25,388 protected baseline hashes** match before and after copying.
  Stock executable/Lua, TGC, V2DLL and profile configuration are retained.
- Target executable passes the reference SHA-256 and all **279 guards**.
- Complete prior 0.1.3 payload and test-guide backup:
  `work/province-rename/ui-universal-020-1021/before-020/`.
- Installation record and exact payload/guide hashes:
  `work/province-rename/ui-universal-020-1021/patch-020-installed.json`.
- Test guide updated at `tgcx_province_rename_test.txt` in the target copy.

Main E:, D: and all other test copies remain untouched. No source/binary
change since the final candidate checks: 51 native checks, 4 package tests,
4 current-V2DLL loader probes, five-base static matrix and TGC scan passed.
No commit, push, tag or publication.

User opens `victoria2.exe` in this copy and selects **Province Rename
(Universal)** plus **TGC v2.5.2 (stable)**. Profile remains
`TGC252_ProvinceRename_20261020`. Follow instructions-rename-020.md sections
2-3. Expect version **0.2.0 Universal**, **56 native signatures OK** and
**Universal Rename ABC icon** in Logs/tgcx_province_rename.log.

0.2.0 appearance/input placement, full persistence/lifetime, other-base
gameplay, ATCU coexistence and multiplayer remain player-test follow-ups.
The earlier accepted 0.1.3 short behavior is retained as distinct evidence.
