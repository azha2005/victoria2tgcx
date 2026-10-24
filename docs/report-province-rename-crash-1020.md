# Province Rename 0.1.1: construction crash and ATCU pitfall audit

2026-10-20. User reports a crash after selecting a province and clicking its
name. The new TGC2.5.2/current-V2DLL test copy ran Rename0.1.0. **0.1.1 is
built, checked and installed; its in-game retry remains pending.**

## Evidence and correction

Full Rename and V2DLL logs plus the failing plugin were preserved under
work/province-rename/crash-0100-1020/. V2DLL5.98 reports C0000374 heap
corruption on the UI timer thread36324 at23:44:48.737. The stack reaches
the named-root factory, native button builder and a scalar destructor.
No Rename queued action occurred: the failure precedes a simulation command.

The generated opening button and its label both used tgcx_rename_open.
Apply and Cancel repeated the same defect. Native FUN_009CFF20 resolves
the button definition by name, then invokes slot11. The colliding type's
slot11 is the thunk at VA0x00A5A430/RVA0x65A430: subtract this by0x64 and
jump to the scalar destructor at VA0x00A59540/RVA0x659540. That destructor
frees the field at+0x144, matching the failing stack. The name lookup is
not type-safe; a syntactically valid GUI file can therefore corrupt memory.

0.1.1 gives all three labels distinct *_label identifiers, retains the
original localisation keys and uses allwaystransparent=yes so their text
does not intercept button input. Packaging rejects duplicate GUI names and
opaque text labels. Before constructing either root, the plugin resolves
all three button definitions and requires the CEU3ButtonType vtable at
RVA0xA08D2C. A mismatch logs its key and refuses construction.

An isolated fixture runs the actual reference-image button builder twice:
the proper definition reaches the button-creation boundary; the collided
definition reaches the real native subtract-this destructor thunk. The
destructor endpoint is intercepted before it frees anything. The new
preflight rejects the second case. This independently exercises the faulty
native dispatch rather than merely asserting the corrected generator text.

## Audit requested by the user

Read docs/pitfalls.md, particularly P27/P31, P40-P45 and P47-P58, and
compared the implementation with ATCU's accepted native wrappers.

| Area / lesson | Result |
|---|---|
| Allocation, P45 | Native clone already establishes0x5C; added a byte guard on its actual push/new site, RVA0x1802BD. Fixture now compares the allocated size to that native immediate, not a second hardcoded size. |
| Native ABI / strings, P27/P31 | Retain measured constructor RET4, button-builder RET32, pointer-based borrowed string inputs, owned command copy and native destructor. Thunks preserve flags/registers/FP/SSE. No guessed tooltip return object. |
| Observer storage | Retain the measured44-byte Politics observer ABI; storage now uses11DWORDs for natural alignment. |
| Selection / lifetime, P44/P47 | Lookups target direct owning roots. Added a click-time root identity/type check, closing stale Apply between a root replacement and the next100ms poll. Existing timer root replacement and destructor cleanup remain. |
| Registered roots, P54 | Keep native hide plus deferred subtree release slot20; no scalar root deletion. |
| Position / visibility, P40/P42/P56/P57 | Keep owner-chain translation, promotion on opening/reopening and secondary subtree Show on transitions. Added bounded construction/initial-placement/editor-open logs for diagnosis. |
| Land meaning, P58 | Nonzero map+0x2A remains LAND. Existing native semantic evidence retained. |
| Loading / base, P48-P52 | Canonical mod/TGC, current per-active-mod V2DLL, independent profile, original Lua, additive Rename GUI. No generated defines file was present in the Rename package during this audit. |
| Script / command boundary | No generated rename script, sidecar name persistence or direct simulation writes. Apply still queues native command0x707. |

These checks do not establish real input focus, display layering, sustained
GUI collection, save/reload or multiplayer behavior. Those remain player
checks. No ATCU code or existing ATCU installation changed.

## Validation and installation

- Reference executable and **264 guards pass**: unchanged179 Army runtime,
 44 prior research and41 separate Rename guards. TGC2.5.2 scan passes.
- Win32 Release build passes with warnings as errors; **37 native checks**,
 **4 package tests** and **4 current-V2DLL loader probes** pass. Loader
 probes deliberately refuse an unsupported host before patching.
- ZIP work/province-rename/candidate/Province-Rename-0.1.1-test.zip:
 11files,178969bytes; SHA256
 3dc493740fe4d4e3def6661beb571ca769177c9a895a8b153fadb03876a0ad04.
- Plugin SHA256:
 feb9467fcec7ff57cb6ad71de619f044f6b08d434501838757d38b825d2b1866.

After the user confirmed the game fully closed following the crash, the
ToolHelp process gate found no game. Only the Rename GUI, version marker,
plugin and test guide changed in work/Victoria-II-Province-Rename-TGC252.
All10package file hashes match0.1.1; **25,388 protected files** match their
original baseline before and after the patch, including TGC/base/V2DLL/Lua.
Old changed files are in crash-0100-1020/before-011; patch-plan.json and
patch-installed.json record the checks. Original stage manifest is historical
and retained. No game was launched or terminated by the agent.

Next: instructions-rename-011.md. Verify selection/open/close first, then
input and command behavior. Do not call the crash fixed in game until the
retry passes. Prior reports remain historical and are not rewritten.
