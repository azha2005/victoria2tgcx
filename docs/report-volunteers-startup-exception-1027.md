# VOL-0: original startup exception analysis

2026-10-27. The original PID 33944 V2DLL logs are preserved in
`work/volunteers-0001/agent-test-1027/startup-crash-33944/`. They record an
earlier `E06D7363` C++ exception before the final `0x40000015` termination.
The stack contains a `std::locale::_Locimp` object, but its presence does not
identify the exception type or establish a locale failure.

With executable image base `0x00230000`, stack return addresses `0x007C2A79`
and `0x007AAACA` map to RVAs `0x592A79` and `0x57AACA`, or preferred-image VAs
`0x00992A79` and `0x0097AACA`. At `0x00992A74`, the native code calls the C++
throw routine with throw-info VA `0x00E6FC00`; the first address is its return
address. The throw-info's catchable-type array resolves to MSVC type descriptors
`.?AVCFileException@@` and `.?AVCException@@`. The enclosing native code contains
the strings `virtualfilesystem.cpp` and `Could not open file: `. The second
stack address is the return from a call into that throwing function. These are
static observations against the checked executable, not proof of the missing
path. The available crash record does not identify the filename.

The next diagnostic is to inspect the exact game filesystem error or run a
controlled comparison with the same executable, mod arguments and working
directory outside the original restricted command environment, with logs kept
separately. A permission, missing-file, mod-path or environment cause remains a
hypothesis until evidenced. The observer initialized, but no tick sample or
gameplay acceptance was obtained. The original crash report remains unchanged.
