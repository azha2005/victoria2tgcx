# NEXT - what comes next

2026-10-24 (74). **TPL-8 Non-core only built in Rust; 0.11.3 candidate ready.**
Installed in `work/Victoria-II-TGC252-own-loader` (backup
`work/noncore-0113/before-0113/`). **Passed in game** (user): Non-core
recruit, Conform detach/build, resume after load and merge; v2 untouched.
Not committed. Session 73 NEXT
archived verbatim in docs/archive/sessions.md.

## Current scope and evidence

- **All new DLL code is Rust** (user, 2026-10-24): `rust/tgcx/`, in
  fedacking's style. The C++ DLL is not extended; only its generated headers
  follow. These headers now describe the 0.11.3 GUI. Do not rebuild the C++
  DLL from them for a release: its Non-core box would do nothing.
- TPL-8 (D19): third exclusive box in Templates and Conform; Conform uses two
  lines and shows 6 Changes rows (was 7), with overflow in the summary. Settings
  in `tgcx_templates_v3.txt`; v2/v1 are imported, never rewritten.
  `docs/report-tpl-8-noncore-1024.md`.
- Candidate `work/noncore-0113/ATCU-noncore-0.11.3-test.zip` (`9801d0b1...`):
  the **first packaged Rust logic DLL** (`4960aa2f...`), with the accepted C++
  loader. Previews: `mockups/5-templates-noncore-1024.png`,
  `6-conform-noncore-1024.png` (Arial, not captures).
- PR #3 mobo stacks merged at `abc6806`. Player crash report on a low-pop
  Recruit is not reproduced; reporter's `tgcx.log`, ATCU version and mod list
  are still awaited.
- The release is still C++ v0.0.2. The Rust DLL is not deployed or released.

## Next session in order

1. **Small look fixes seen during TPL-8 (pre-existing):** long template names
   wrap over their "N regiments" line in Conform's list; the left-out
   Changes title is pale yellow on parchment.
2. **TPL-11 Build Army gathers into one army.** Reuses Conform march/merge,
   TPL-7 saved orders and the mobo assign-and-march path. Open [user]: the
   gathering province (player pick, capital, largest source province or the
   native rally point); regiments without a land route (D17: leave them).
3. **TPL-9 Autodivide.** Base: Conform's detach (Create + Transfers together,
   P51). Open [user]: the rule (single regiments, N equal stacks, template
   copies), the button's place, who keeps the leader.
4. **TPL-10 Carpet siege.** Needs TPL-9 plus one safe-path Move per stack.
   Open [user]: which provinces, regiments per province, one-off click or a
   running order that re-targets.
5. **TPL-12 Replace depleted regiments.** Needs research first: the regiment
   strength field and the native disband command (R5). Open [user]: the
   threshold, disband vs detach to a reserve, build-first order, separate
   button or Conform option.

Every Army card is Rust, needs its mockup (R10) and its [user] questions
answered (R9) before code. Full card text: docs/archive/sessions.md,
"Next up after 0.0.2".

Alongside, when selected:

- **RUST-CHECK:** `docs/instructions-rust-check.md` (Bill, favours, the
  rest of Conform; the TPL-8 check covers part of it).
- **RENAME-026:** `docs/instructions-rename-026.md` (C++, packaged; optional
  final 2px look check). No publication implied.

Next pitfall remains P64.
