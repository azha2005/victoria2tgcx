# ATCU

**Army Template and Conform Universal** adds army templates and a Conform
button to Victoria II 3.04. Build a composition once, recruit it, or bring
an existing army into that composition.

The same mod folder works with the active base. Short gameplay checks have
passed on vanilla, HPM 0.4.5, Divergences of Darkness Fan Fork and TGC 2.5.2.
TGC was also checked with V2DLL 5.82. Other mods remain untested.

## Release 0.0.2

Prepared from internal build 0.11.2 (the DLL reports that build in its log):
culture and continent filters, and Conform orders that survive a load.

- [README.txt](docs/release-0.0.2/README.txt): install, controls, known
  limits, bug reports and credits (the one file shipped in the ZIP)
- [Release notes](docs/release-0.0.2/RELEASE_NOTES.md)

Release 0.0.1 (internal build 0.10.4): [README.txt](docs/release-0.0.1/README.txt),
[release notes](docs/release-0.0.1/RELEASE_NOTES.md).

The download is files only. Players do not need an installer, Python,
PowerShell, batch files or development tools. Victoria II and any base mod
must already be installed. The original game executable and Lua library
are not included.

## Source and builds

The current source checkpoint is `1482eee`. This repository also contains
earlier TGCX work on Politics, which is not enabled by the ATCU Army package.
The internal `tgcx_` names remain for compatibility with existing templates
and installations.

Maintainer packaging instructions are in
[RELEASE-0](docs/instructions-release-0.md). `NEXT.md` tracks current work.
The project's own license remains undecided for this candidate. Credits
and source attribution do not assign a license to third-party material.

## Experimental Volunteers prototype

The independent [Volunteers source](rust/volunteers/README.md) retains donor
ownership and player command while assisting a recipient war. Sending, battle
and siege progress have passed player checks; full lifecycle/vision/occupation
credit validation is incomplete and multiplayer sending is refused.0.0.11 adds
measured helper optimizations. This is a source checkpoint; private game-derived
art and test packages are not included in the repository.
