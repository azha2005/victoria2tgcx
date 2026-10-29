# VOL-0: failed native write path

2026-10-27. A read-only Ghidra decompilation of the two original PID 33944
stack locations completed after `report-volunteers-startup-exception-1027.md`.
Its private output and Ghidra log are in
`work/volunteers-0001/agent-test-1027/startup-crash-33944/`.

The throw at preferred VA `0x00992A74` is in `FUN_00992740`. In its write-mode
branch, the function chooses `PHYSFS_openWrite` or `PHYSFS_openAppend`; if the
result is null, it obtains `PHYSFS_getLastError` and constructs and throws
`CFileException`. The original return address belongs to that throw. The
specific file path, last-error text and which of the two operations failed are
not present in the preserved crash record. This supports investigation of
restricted filesystem access, but does not prove the precise cause.

The separately authorized same-arguments desktop launch produced a targetable
`Victoria 2` window for the disposable executable. Two supported computer-use
state captures failed with `window capture timed out`, even after refreshing the
window selection. The skill's recovery guidance requires stopping after that
failed retry; no blind input was sent. This window's existence is not campaign
or observer acceptance.
