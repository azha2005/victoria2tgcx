# Volunteers 0.0.17

Experimental SP and whole-army MP prototypes. Use `mode=mp` for explicit MP
admission; `mp_probe` remains diagnostic only. No unknown session becomes SP.

MP policy is derived from native-executed per-country saved journals at HEAD
execution and before the native hourly command. All peers need fresh current-load
HELLO receipts, the same DLL/policy fingerprint and a complete cross-checked native
human roster. Each admitted local donor alone queues its journal and unit orders.
Merged immutable policy retains per-donor campaign IDs and is never persisted.
MP sends whole existing armies only; native split controls can prepare a small
stack first. This avoids unsynchronized allocation of a new army ID.

Native hooks require exact relocated signatures. Transparent host-role probes
are recognized by their bounded instruction and original-continuation contract,
independent of DLL hashes, names or private RVAs. Compatible existing hooks remain
in place; other plugins are not refused by their mere presence. Unknown changes
report hook owner/path and a refusal reason rather than bypassing their code.
UI gates are independent so a gameplay mismatch can remain diagnosable. Our
CSetVariableCommand clone wrapper preserves sender metadata with its inspected
ECX / seven opaque stack words / EAX / RET1C ABI; flags/sequence stay native.

`tools/build_volunteers.ps1` runs x86 tests, strict Clippy and release build.
`tools/prepare_volunteers_mp.py <new-output> --gameplay` produces a plain ZIP,
with no installer, EXE, command files or scripts. Read its README for paired tests.

Startup and source fixtures are separate from actual multiplayer acceptance.
No claim that the original OOS is fixed until paired movement/battle/siege/return,
save/reload and engine OOS traces agree. No local maintenance or CRC bypass.
