# VOL-0: command-launch startup exception

2026-10-27. The user authorized a direct command launch after the desktop
launcher attempt. The parent launched the disposable copy with working
directory `work/Victoria-II-Volunteers-0001`, PID 33944, and arguments
`-mod=mod/tgcx_volunteers.mod` and `-mod=mod/TGC_v2.5.2.mod`. Exact metadata is
in `work/volunteers-0001/agent-test-1027/command-launch.json`.

The user supplied an image of an application error reporting exception
`0x40000015` at `0x00000000008E495E`. When this agent checked, PID 33944 had
already exited and neither a Victoria II process nor that error window appeared
in the fresh computer-use window inventory. The agent did not dismiss the
dialog or invoke the debugger. No campaign or observer tick was reached.

The complete volunteer log, retained as `startup-volunteers.log` in the same
evidence directory, shows `Supported EXE`, image base `00230000`, and
`observer_installed=true protection_restored=true`. The loader reached plugin
initialization; no subsequent tick sample is present. This does not prove the
plugin caused or did not cause the failure.

Assuming the log's image base is the one for the reported address, subtraction
maps `0x008E495E` to executable RVA `0x6B495E`, preferred-image VA `0x00AB495E`.
The reference executable disassembly immediately before this address pushes
`0x40000015` and calls another routine; the reported address is the subsequent
instruction. The termination path is identified, but its caller and underlying
cause are not. The disassembly is preserved as
`startup-exception-disassembly.txt`. A read-only recent Application event query
for `v2game` or PID 33944 returned no matching event. This is not proof that
Windows recorded no related event under another description.

No further launch was attempted by this agent. A controlled diagnosis must
inspect base and loader startup evidence, and compare a suitable disposable
control before attributing the error or changing the observer. Keep the main
installation, existing test copies and user saves untouched. Initialization,
three-day samples, selection change, save/reload and all gameplay acceptance
remain open; sending, control and recall are not implemented.
