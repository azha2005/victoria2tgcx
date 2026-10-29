# Volunteers: background-only work and ownership follow-up

2026-10-27. The user authorizes the agent to operate its own test, with the
explicit condition that it must not take over the desktop while the user works.
This supersedes the earlier user-run-only wording for this volunteer task.
No host keyboard/mouse input or foreground game launch is authorized under
that condition. Background builds, fixtures and headless Ghidra remain usable.

## Test environment

Prepared a disposable workspace copy at work/Victoria-II-Volunteers-0001 from
the existing PRU/TGC252 test copy. Reference EXE, loader and V2DLL core hashes
match the source copy; the volunteer DLL matches the checked candidate. The
test-only TGC descriptor uses TGC252_Volunteers_20261027 as its user_dir.
The main E: installation and previous test copies were not edited.

Before the desktop restriction was stated, an attempt to open the disposable
launcher through the computer-use tool timed out with "Computer Use app
approval timed out". This was not a game test or an automatic approval rejection.
No keyboard/mouse input was sent. A subsequent process check found no v2game
or victoria2 process. No launch was retried after the restriction.

The available desktop-control surface shares the user's desktop. QEMU is
installed, but the bounded Downloads/Documents image search did not find an
identified Windows guest prepared for this game. An existing unrelated Linux
VM was not launched or changed. This does not prove that no usable VM exists
elsewhere. No guest OS, driver or security setting was installed or changed.
An unattended game acceptance run is still outstanding.

## Fresh static ownership evidence

Reference check_exe passes. A readOnly/noanalysis Ghidra run on the existing
workspace project exported 19 successful pseudo-C/assembly pairs, references
and direct call sites to work/volunteers-control-1027. Imported EXE SHA matches
62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648.
All findings below are S, inspected instructions; none is new runtime evidence.

| Finding | Evidence, Ghidra VA / RVA |
|---|---|
| Base unit initialization clears both owner index +0xC4 and affiliation index +0xCC | 0x005C5F05 / 0x1C5F05; 0x005C5F11 / 0x1C5F11 in the earlier army raw export |
| Native owner setter updates every regiment's +0x60/+0x64 and army +0xC0/+0xC4; it replaces affiliation +0xC8/+0xCC only if the previous affiliation index is zero or equals the resolved special-country index | 0x005C93B0 / 0x1C93B0, stores/condition at VA 0x005C93EE..0x005C9432; ESI army, stack tag/index, RET8 |
| Create command execute initializes ownership from its own +0x3C/+0x40 country reference | 0x0057DBF0 / 0x17DBF0, native setter call at VA 0x0057DD1B / RVA 0x17DD1B; ECX command, plain RET |
| Add-regiment copies destination army ownership and sets the parent pointer; its inspected body does not copy the source army's affiliation | 0x005CB1D0 / 0x1CB1D0; ESI destination, stack regiment, RET4, prior army raw export |
| Can-transfer inspects unit categories, regiment transferability, source/destination state and province. No country-owner comparison occurs in this inspected body | 0x00513F30 / 0x113F30; EDI destination, stack regiment, AL, RET4 |
| Normal pop reconciliation compares source-pop province owner with army affiliation +0xCC | 0x005C8330 / 0x1C8330, comparison at VA 0x005C8662 / RVA 0x1C8662; mismatch reaches split/remove branches |
| Country-list insertion calls the owner setter as well as linking the army; invoking it is a simulation mutation, not a read-only list helper | 0x00513C80 / 0x113C80, setter call at VA 0x00513D10 / RVA 0x113D10; EAX army, EDI country, plain RET |

Consequently, creating a fresh B-owned army and transferring A's regiments
does not by itself establish donor affiliation. The inspected native sequence
initializes +0xCC to B. This explains a concrete maintenance risk; it does not
prove that every foreign-pop regiment will be removed on the next tick.

There are two research candidates, neither enabled:

1. A queued engine/script route that changes an already A-affiliated dedicated
   stack to B while retaining affiliation, and updates the country lists safely.
   The setter's conditional behavior supports this hypothesis, but a suitable
   command adapter remains unestablished. Calling the setter or list helpers
   directly does not satisfy R5.
2. Native Create(B) + Transfer commands, with narrowly scoped volunteer-aware
   pop reconciliation. Can-transfer's inspected body supports investigating
   cross-owner transfers, but does not prove queue acceptance. Any exemption
   must affect only the foreign-owner comparison for verified loans, preserving
   pop validity/death/support, losses and ordinary non-volunteer behavior.

The owner-setter caller survey found construction, loading/creation,
country-wide and pop-reconciliation paths. It did not establish a dedicated
single-army loan ownership command. Give Unit Command still delegates at the
country-relation level; it is not an ownership handoff for one volunteer stack.

No new runtime hooks, command constructors, simulation writes or UI changes
were added in this follow-up. The checked 0.0.1 ZIP remains observe-only and
unchanged. Its earlier build/fixture results remain valid but are not gameplay
acceptance. See instructions-volunteers-isolated.md and the updated VOL-1 card.
