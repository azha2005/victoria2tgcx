# Volunteers prototype

Independent Rust Win32/x86 V2DLL plugin for the reference Victoria II3.04 EXE.
Current source:0.0.12. Donor owns/commands/supplies selected land brigades while
borrowing recipient war context only for registered volunteer operations. Native
GUI, unit commands, save-backed mission journal, scoped combat/siege/access and
local display vision are implemented. User checks confirm sending, battle and
siege progress on the isolated TGC2.5.2/V2DLL5.98 setup. Full prototype acceptance
is incomplete; MP sending is deliberately refused.

Read ../../NEXT.md and ../../docs/volunteers-design.md before changing behavior.
The guide ../../docs/report-volunteers-performance-probes-1029.md records the
remaining lag, idle-wrapper measurements and expensive V2DLL diagnostics found
enabled in the isolated test copy. Sampled callback timing accompanies weekly
observer reports; gameplay updates stay daily. No simulated benchmark
ratio is a game FPS claim.

Build on Windows with Rust i686-pc-windows-msvc/MSVC x86. From repository root:

    ./tools/build_volunteers.ps1
    python tools/test_volunteers.py

The build script enters this directory so .cargo/config.toml selects the correct
32-bit target. For an explicit controlled release helper benchmark, enter this
directory and run:

    cargo test --release --offline perf_tests::helper_benchmark -- --ignored --nocapture --test-threads=1

It owns synthetic memory; it never opens or changes a game process. Normal tests
leave this timing benchmark ignored to avoid noisy routine measurements.

The full inactive-wrapper measurement also runs explicitly:

    cargo test --release --offline idle_pair_wrapper_benchmark -- --ignored --nocapture --test-threads=1

Private packaging from repository root:

    python tools/prepare_volunteers.py work/volunteers-local

Packaging expects the local prepared reference game/art path configured in
../../tools/tgcx_volunteers_gui.py. It validates and generates new tgcx_ GUI/assets
from installed game art. Outputs contain private game-derived textures and stay
ignored; no game assets, executables, raw Ghidra exports or release binaries are
part of this source checkpoint. Build/byte guards refuse unsupported executables.
Install only into a backed-up prepared copy after normal game exit; the package
README gives its explicit observe/preview/prototype modes and native-loader setup.
