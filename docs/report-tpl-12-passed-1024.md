# TPL-12: 0.11.15 passed in game; native name box covers Disband depleted

2026-10-24, session 81. User on 0.11.15: **"everything worked"**, with one
remaining issue (below). Log preserved: `work/tpl12-01115/passed-tgcx.log`.

## Evidence (tgcx.log)

- Main-menu load seen with the save's date (P65 fixed):
  `load seen ... player 0->65 date 59883360->59884296 step 0`, then
  `5 saved orders resumed`, `2 replacements resumed`.
- Stranded Lubny regiment: `stack 8768 joins 415`, then
  `1 replaced regiments leave stack 415`, `1 of 1 extras arrived`.
- New replacements on stacks 493, 426 and 480; 426 and 480 each joined,
  then their old regiment left and arrived in its own stack.
- Second reload through the main menu: `2 pending orders forgotten`
  (real), `6 saved orders resumed`, `3 replacements resumed, **2 old
  regiments to detach again**` (0.11.13 merged-state path), both arrived.
- No regiment disbanded; no false load line.

**TPL-12 single-player functional check passed on TGC v2.5.2.** Not
covered: multiplayer, other bases, same-date or short-forward reloads.

## Remaining issue: name box over Disband depleted

User: hovering Disband depleted from its centre leftwards shows the unit
name tooltip ("Name of Unit") and the button cannot be clicked.

Cause: TGC's own `unitpanel.gui`. The `unitname` edit box sits at x=38,
size 190 (ends at 228); `disbandtoosmallbutton` is at x=214, 20 wide.
The box covers the button's left 14px and takes the click. Vanilla has
the same overlap at 4px (size 180, ends 218). ATCU only reads that
button's visibility; it does not move or resize either control.

Fix options (user's choice, not started): leave it (TGC behaviour, click
the right side), or have ATCU narrow the name box's hit area at runtime
so it stops at the button while the button is shown. The latter needs
the edit box's live size field and its hit test checked first (P61: live
size fields differ from definition fields); text past the new edge would
be clipped.
