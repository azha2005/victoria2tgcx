# ARMY-HANDOFF: targeted research export

2026-10-26. Prepared the army/regiment research requested by the user's friend.
Local package: `work/army-handoff-1026.zip` (16 files, 162744 bytes).
SHA-256: `c950fc8b40bfe805bb966d420d089b611cf80c3b89a3e36b7c6da77f5ba71d43`.

21 functions freshly exported with Ghidra 12.1.4, with corresponding disassembly,
from a copied project opened read-only. Named CArmy/CRegiment data types are
absent; the package explicitly supplies partial evidenced field maps instead.
Includes creation/transfer command constructors, execution and validity,
internal add/remove helpers, native UI callers, country AddUnit/RefreshSubUnits,
base constructors, recruitment completion, registry lookup and identity lock.
Includes existing C++/Rust wrappers, symbol corrections and executable gates.

Reference executable SHA and all check_exe signatures pass. All 21 functions
exported without missing/failed markers. ZIP CRC and every archived payload
compared successfully against its input. No gameplay, multiplayer, new feature,
installation, release, upload, commit or push. No subagents. Existing dirty work
preserved. Derived output and the copied Ghidra project remain ignored in work/.

Next: no implementation selected; optional package regeneration procedure is
`docs/instructions-army-handoff.md`. Existing NEXT selections remain available.
