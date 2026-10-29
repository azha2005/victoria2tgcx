# Volunteers prototype

Independent Rust Win32/x86 V2DLL plugin for the reference Victoria II3.04 EXE.
Current source:0.0.13 feedback candidate (not yet tested in game). Donor owns/commands/supplies selected land brigades while
borrowing recipient war context only for registered volunteer operations. Native
GUI, unit commands, save-backed mission journal, scoped combat/siege/access and
local display vision are implemented. User checks confirm sending, battle and
siege progress on the isolated TGC2.5.2/V2DLL5.98 setup. Full prototype acceptance
is incomplete; MP sending is deliberately refused.

2026-10-29 loader update: the isolated copy now has CreatineCore 6.36, with
Volunteers 0.0.12 unchanged. Proxy/API fixtures and settings migration pass;
startup is now confirmed in logs and the user accepts general operation/speed.
Full prototype acceptance remains open. See
../../docs/report-volunteers-loader-636-1029.md and its acceptance instructions.

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

Plain player-shareable package (no installer/launch scripts):

    python tools/test_volunteers_share.py
    python tools/prepare_volunteers_share.py <new output folder>

This uses the immutable accepted candidate and generated carrier; see
../../docs/report-volunteers-share-1029.md. No loader distribution or private
logs/saves are included. DLL/game-derived UI pixels remain outside git.

Tester feedback changes and exact retest steps: ../../docs/report-volunteers-feedback-1029.md
and ../../docs/instructions-volunteers-feedback.md. Current plain ZIP uses the
checked0.0.13 candidate, while the accepted local game remains0.0.12. No MP Send
unlock; mode=preview is a read-only multiplayer baseline only.
