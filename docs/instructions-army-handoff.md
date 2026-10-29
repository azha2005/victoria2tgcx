# ARMY-HANDOFF: optional follow-up

1. Read `report-army-handoff-reviewed-1026.md`, then the local revision 2 README
   in `work/army-handoff-1026-v2/`. Use the regenerated main ZIP, not v1.
2. Read 01-LAYOUTS, 02-COMMANDS and 03-EVIDENCE in that order. They supersede
   the older handoff's partial-layout.md. Unknown fields remain unknown.
3. If more fields are requested, identify the exact operation before researching.
4. Run `python tools/check_exe.py` before using addresses. Work from a copied
   Ghidra project in ignored work/, read-only/noanalysis; export targeted code.
5. Use the v2 tools/ExportArmyHandoff.java for raw evidence. Extend guides only
   when instructions/callers establish the claimed byte offsets and ABIs.
6. Recheck the byte-view header with C++17 -Wall -Wextra -Werror -fsyntax-only.
7. Rebuild using the v2 tools/package_handoff.py and verify with
   tools/verify_handoff.py --exe <reference-exe> --zip <output-zip>.
8. Keep derived output/database outside Git. Do not call internal simulation
   mutators directly. New features require selected implementation scope.
9. No external message has been sent; the user can give the ZIP to the friend.
