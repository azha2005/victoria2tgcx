# VOL-0: computer-use recovery

2026-10-27. The user authorized desktop control and requested continued
recovery until the agent can operate the disposable game. PID 34692 exposed a
`Victoria 2` window from the intended disposable executable. The agent could
not obtain a screenshot or observe actionable game controls, so no game input
or campaign acceptance is claimed.

The agent tested fresh window selection, accessibility-only state, a reset and
reinitialization of the JavaScript runtime, explicit activation of the game
window, and the documented CUA application interface. Accessibility returned
only a generic `Victoria 2` pane; game screenshots returned `window capture
timed out: timed out waiting on channel`. The Windows CUA screenshot interface
uses Sky as well and returned the same timeout. A separate standard Explorer
window also timed out after activation and fresh selection. This extends the
observed failure beyond the game but does not determine its cause.

The installed computer-use API documents no alternate screenshot transport or
helper-restart operation. Its recovery guidance says to refresh selection and
retry once after a state-capture failure; the supported paths above were tried
without blind input. Restoring the Windows computer-use capture service or
providing a supported visual observation path is necessary before this agent
can operate the game reliably. Do not infer that a running window, plugin
initialization or a created profile is a passed campaign check.

The private chronological record is
`work/volunteers-0001/agent-test-1027/computer-use-recovery.txt`. No security or
privacy setting was changed, and no game process was terminated by this agent.
