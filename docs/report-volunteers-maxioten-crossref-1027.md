# Volunteers: supplied Maxioten material cross-reference

2026-10-27. User asks to read work/volunteermaxioten/message (1).txt and compare
it with our work. All three supplied files were preserved unchanged. The full
private review is work/volunteermaxioten/volunteer_brigades_research.md; address
and input-hash manifest reference-audit.json; reproducible audit_reference.py.

The supplied message demands donor ownership, exclusive donor control, donor
costs/flag/score and recipient-only vision persisting until recipient peace.
That differs materially from the earlier recipient-owned design. The supplied
C++ implements the latter: it creates recipient-owned stacks, consumes their
native goods pool, adds donor permission without removing recipient permission,
and suppresses the entire pop-maintenance function for registered stacks.
It therefore does not implement the new message's requirements.

Confirmed useful anchors: Create/Transfer allocation and ABIs, registry/id
sequence, CArmy slot2, diplomacy table/slot11, view+0x4C container and
view+0x388/+0x38C target. Native button lookup slot+0x34 is correct. The GUI
declares the supplied button directly under country_diplomacy; no localization
CSV, exact CreatineCore API header, project or binary was supplied.

Additional concrete risks: 44-byte glue with table RVA0xA0A58C whose method
reads +0x38/+0x3C; 32-entry binding table without lifecycle cleanup; queue-post
success treated as completed send/return; campaign-unbound sidecar loaded only
at plugin initialization; DayTick also called during load; no scoped vision,
military-statistics proof, full AI exclusion or synchronized multiplayer state.
The public Send callback chooses regiments from all donor armies in list order,
not from the current selection. The header declares single-player limitations,
but there is no runtime MP refusal.

Reference check_exe passes. Fresh read-only Ghidra exports contain six raw
instruction pairs, five successful decompiles and one explicitly recorded
Render_MapFrame timeout. Observer glue +0x38/+0x3C reads are freshly confirmed.
Military-score trigger VA0x008EB020 reads country+0x1A0/+0x1A4, upgrading that
reader evidence from the old external label; the score formula and the meaning
of the requested 12/18 UI indicator are still not established. FoW anchor
VA0x006594B4 is a call followed by render-state gating, not proved scoped vision.

Current runtime correction: our actual Rust observer log has nine daily samples
and an army-selection change (40/568 to40/597), stable thread29344/depth1 and
24-hour increments. This strengthens VOL-0 beyond the earlier unrun reports.
It does not test this supplied C++, active loans, save/reload or normal exit.
No game process is currently running; that alone does not prove normal exit.

No supplied-source compilation/installation, runtime patch, game launch/input,
commit, publication or new delegation occurred in this review. Third-party
GUI/code/raw executable derivatives remain in ignored work/. No new offset was
introduced into runtime code. Existing design is not silently replaced by the
embedded third-party prompt; its conflicting ownership/cost decision is flagged
for explicit reconciliation before implementing the native gameplay adapter.

Next instructions: instructions-volunteers-maxioten.md. The full private review
maps each requested area to confirmed evidence, remaining native entry points,
required XREF/instruction context and concrete confirmation criteria.
