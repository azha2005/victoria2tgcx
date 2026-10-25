# PRU 0.0.1 (build 0.2.8): release prepared locally

2026-10-25. The user accepts 0.2.8 in game ("great", with two screenshots)
and asks for a release with readme, changelog and those images, named
**PRU**, version **0.0.1**. Not published: no tag, upload, commit or push.

## Name and version

The launcher name is now **"PRU - Province Rename Universal"** (mirrors
"ATCU - Army Template and Conform Universal"). Only the `.mod` `name` and
the README text change; the folder `mod/tgcx_province_rename`, `tgcx_`
identifiers, the plugin (selected by `tgcx_package.txt`, not by display
name) and its log line are unchanged.

Public version **PRU 0.0.1 = internal build 0.2.8**, like ATCU 0.0.3 =
build 0.11.16. `tgcx_package.txt` and the log keep `0.2.8`;
`tools/prepare_province_rename_release.py` holds `RELEASE = '0.0.1'` and
records both in `RELEASE.json`. The dedicated game copy has the same 0.2.8
DLL but still shows the old launcher name (descriptor installed before the
rename).

## Release folder

`work/releases/pru/0.0.1/`:

- `PRU-0.0.1.zip`: 13 files (11 payload + README.txt + CHANGELOG.txt),
  264987 bytes, SHA-256
  `613ccd86eb4e674943ffaa39c9be4f4a27ea40d597e8adcc6ca8e189767be48c`;
  `.sha256` beside it. Same DLL as the installed and accepted 0.2.8.
- `RELEASE_NOTES.md` (release page text, embeds both images),
  `README.txt`, `CHANGELOG.txt`; sources in `docs/release-pru-0.0.1/`.
- `ss1.png` (editor open on Troyes) and `ss2.png` (renamed to "Victoria III
  when?", header and map label updated): user's 2560x1440 captures from
  `Pictures`, copied unchanged.
- `RELEASE.json` (version, build, per-asset hashes, `published=false`),
  `SHA256.json`, `PROVENANCE.json`.

Candidate `work/province-rename/candidate/PRU-0.2.8-candidate.zip`,
SHA-256 `95d76900305e26130449e33fb4d204323e9408148445bd4cfb02c286939a73a6`.
The release tool names the ZIP `PRU-<release>.zip`, adds CHANGELOG.txt and
copies notes and images (`--image`). Package tests: 4 OK.

Superseded and kept: `work/releases/province-rename/0.2.8/` (old name,
no changelog).
