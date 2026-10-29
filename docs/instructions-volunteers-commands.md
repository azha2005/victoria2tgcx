# VOL-1: command and control prototype

Superseded by the user's donor-ownership/player-control choice on2026-10-27.
Follow instructions-volunteers-donor.md. The historical recipient-ownership
investigation below must not be used as current implementation instructions.

2026-10-27. User has selected the independent volunteer submod. All new DLL
code is Rust in rust/volunteers; no ATCU/PRU feature changes are required.

1. Read volunteers-design.md, report-volunteers-foundation-1027.md and the five
   local volunteer research guides. Run check_exe before address-dependent work.
   Also read report-volunteers-background-1027.md: 19 further exports are in
   work/volunteers-control-1027. Do not repeat that caller survey blindly.
2. Review the VOL-0 log from instructions-volunteers-check.md when available.
   Resolve context/selection/date/thread mismatches before adding runtime writes.
   Static command tracing below can continue independently of that game check.
3. Trace a native engine-command route for creating recipient ownership and
   restoring donor ownership. Internal SetOwner at RVA0x1C93B0 is not a command
   itself. The fresh caller survey has not found a dedicated loan command.
   Investigate either an existing command/script handoff preserving donor
   affiliation, or Create(B)+Transfer with a narrow, proved loan-specific
   pop-owner comparison exemption. Cross-owner queue acceptance is unproved.
   Keep derived evidence in ignored work/.
4. Explain the original/affiliation block at army+0xC8/+0xCC from native writers
   and maintenance, not a guessed controller name. Trace foreign-pop handling
   for both recipient-war and recipient-peace branches. Preserve pop death,
   support and losses; do not skip the entire maintenance routine for a loan.
5. Prove army-specific control and list inclusion for only registered dedicated
   stacks. Preserve original overlord behavior and non-volunteer UI behavior.
   Include map selection, order dispatch and queue authorization, not just slot2.
6. Add a native command adapter only after the required ABIs/signatures are
   established. Implement receipt verification; queue acceptance is not arrival.
   Track original brigade/pop identities, campaign identity and return state.
7. Wire the native UI from volunteers-layout.svg only when the command gates
   work. The chooser is a design proposal; consent, travel/delay and entry point
   need concrete decisions before those mechanics are finalized.
8. Run tools/build_volunteers.ps1, tools/test_volunteers.py and appropriate new
   native-adapter tests. Stage a new version/output folder; never overwrite the
   prior checked ZIP or install over the user's active game.
9. Agent-run one-brigade acceptance once implementation is ready, following
   the current desktop permission in instructions-volunteers-desktop.md:
   donor stays neutral and retains command,
   recipient-enemy combat, several maintenance days, recall/peace, destruction,
   save/reload. Stop on identity/owner/pop inconsistencies and retain evidence.
10. Multiplayer remains a separate card under D5. The pure model rejects it;
    identical local registries/sidecars are not a synchronized implementation.
