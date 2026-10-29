# VOL-0: authorized desktop test attempt

2026-10-27. The user subsequently authorized shared desktop control for this
test, superseding the isolation restriction recorded in
`report-volunteers-agent-test-1027.md`. **No campaign test was completed.**

The agent checked for existing `v2game` and `victoria2` processes; none was
returned. With the supported Windows computer-use API, it attempted to launch
`work/Victoria-II-Volunteers-0001/victoria2.exe`. The call returned:

> launched app did not expose a targetable window

Subsequent read-only process and computer-use window checks returned no
Victoria II launcher or game. The earlier approval timeout is distinct from
this result; neither error is an automatic approval rejection. No mod selection,
campaign, game log or screenshot was obtained.

To investigate whether opening the executable from its own folder would help,
the agent launched File Explorer with the supported API and selected an
Explorer window from its returned window list. State capture failed with
`FrameArrived timed out: timed out waiting on channel`. After refreshing the
window list and reselecting, a second capture failed with `window capture timed
out: timed out waiting on channel`. The computer-use recovery guidance requires
stopping after a failed retry. No keyboard or mouse input was sent, and no
game process was terminated. The Explorer window could not be inspected, so
the folder-context launch was not attempted. The exact cause of either error
is unconfirmed; no conclusion about the candidate's game compatibility follows.

Prior hash, configuration and executable checks are recorded in the earlier
report and its ignored evidence folder. The requested initialization,
three-day observation, army change, save/menu/reload and normal-exit checks
remain open. No loader coexistence, multiplayer, sending, donor control or
recall has been established.
