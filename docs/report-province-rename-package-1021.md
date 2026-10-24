# Province Rename 0.2.6: final ABC placement and local ZIP

2026-10-21. User accepts 0.2.5: "works awesome", requesting a small ABC
placement improvement and initially a release. After reviewing the ready
package, the user explicitly changed the scope to **"dont release it yet,
just give me the zip"**. No publication, remote push or source commit.

0.2.6 raises the 24x24 ABC button **2px** and changes the gap after the
measured province name from8px to **10px**. Bounds/clamping, font-dependent
name measurement and translated native coordinates remain intact. The
accepted dialog, Garamond input, caret, native command and other behavior
are unchanged. Native geometry expectations and generated local preview/
board were updated together. Previous board retained verbatim in
`docs/archive/province-rename-layout-025.svg`.

Unedited acceptance screenshot and 0.2.5 log are under
`work/province-rename/release-026-1021/`. The log confirms two native renames
for province416, header/readback updates and reopen, with78guards and the
single-origin input correction active. Player acceptance covers short
0.2.5 behavior; the final2px adjustment itself still awaits visual retry.
Full save/reload/lifetime/ownership limits, other-base gameplay, ATCU
coexistence and multiplayer remain unproven.

## Validation

- Reference executable gate:301guards, unchanged78Rename runtime guards.
- MSVC Win32 build,66native checks,4package checks and4actual V2DLL loader
  probes pass. Font/sprite/cursor checks remain static across five bases.
- Updated native anchor regression covers the final2px rise/extra gap,
  translated parents, long-name clamping and fallback placement.
- No new addresses, hook, asset extraction or pitfall; next P64.

The local ZIP is prepared by `tools/prepare_province_rename_release.py`
from the checked immutable candidate. The tool verifies CRC, safe unique
paths, exact candidate manifest, matching plugin and package version. It
replaces only README.txt; all11mod payload members remain byte-identical.
The written archive is reread and compared. Sidecars preserve per-file
hashes, native-art provenance, source hashes and the unpublished identity.
No V2DLL binary, executable, base GUI or base data is included.

## Deliverable

`work/releases/province-rename/0.2.6/Province-Rename-Universal-0.2.6.zip`:
**12files,264502bytes**; SHA-256
`fcc68a16c0987c5f947d8ada1fde7060c85a2e2b3f51da4d703f8ed05055ce9f`.
Adjacent `.zip.sha256`, `SHA256.json`, `PROVENANCE.json` and `RELEASE.json`.
Identity explicitly records `published=false`, no destination/source commit;
checkout HEAD is only contextual, not claimed to contain the new sources.
README/notes source: `docs/release-province-rename-0.2.6/`. Native derived
pixels and binaries remain ignored under work/.

Candidate ZIP: `work/province-rename/candidate/Province-Rename-Universal-0.2.6-test.zip`,
12files; SHA-256
`ac0838288bcb2d53a3539c4780430ca421e97575ace771d43e39bc699b3185fb`.
Native-font preview: `work/province-rename/candidate/layout-preview-026-native.png`.
Preview is generated, not a gameplay capture.

## Installation and handoff

User freshly confirmed **Yes, closed normally** for this exact update.
Process gate clear throughout installation; only
`work/Victoria-II-Province-Rename-TGC252` updated. All11payload hashes and
25,388protected hashes verified; protected files unchanged. Full0.2.5
payload/guide rollback in `release-026-1021/before-026/`, exact record
`release-026-1021/patch-026-installed.json`. Earlier backups retained.
Stock Lua, canonical TGC, V2DLL5.98/cdad55c7 and independent profile retained.

No game launch/termination, other-copy/main E:/D: update, publication,
commit, push or memory update. No new licence choice. Follow
`instructions-rename-026.md` for optional final placement/player checks.
Publication is explicitly deferred until the user asks again and selects
the destination; the requested result of this session is the local ZIP.
