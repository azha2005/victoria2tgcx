# Volunteers Send refusal: France at war

2026-10-29. User accepts the native appearance and requests instructions because
Send shows "These brigades cannot be sent now." No UI input or process writes
were performed by the agent. The current game remains running and unchanged.

Read-only process inspection after check_exe ALL OK: isolated E: copy PID29728,
loaded base0x00EB0000, Volunteers0.0.7. Log confirms prototype requested, all five
scoped hooks installed, native load finalization epoch1 valid. The displayed
message is the validate_send refusal branch after native/session admission.
France index80 has at-war byte+67C=1; relation+34 to ALD/index480 is nonzero.
Effective TGC localisation00_countries-names.csv names ALD Algeria. RGS/index648
is at war and FRA->RGS hostility is0. The sender-at-peace requirement is the
current blocker. Donor armies inspected were idle, donor-owned and donor-affiliated.
Raw snapshots/scripts: work/volunteers-0007/send-check-1029.

Instructions: finish France's war or test a peaceful donor; choose a nonhostile
recipient at war, an idle land army and up to5 brigades, then Send and let the
simulation advance for queued commands/receipts. A verified land route is also
required. France-to-RGS from Europe has no oversea transport automation; manually
position/transport the source army on the recipient's continent first, or choose
a nearby land-war case. These are prototype limits, not a reason to bypass gates.

The screenshot/read-only state supports native UI and policy refusal evidence.
It does not establish successful Send, battle, costs, return, reload or MP/OOS.
Generic refusal wording and blank army labels remain UI feedback improvements.
No new build/install, campaign action, desktop input or behavioral suite occurred.
