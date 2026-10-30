//! Standalone volunteer development build. Observer/preview by default, with
//! an explicit experimental SP prototype. Exact native/session/load/registry
//! admission is required; multiplayer is unsupported. Runtime acceptance open.
pub mod app;
pub mod combat;
pub mod engine_store;
#[cfg(any(test, all(windows, target_arch = "x86")))]
mod entry_visibility;
pub mod lifecycle;
pub mod mission;
pub mod model;
pub mod mp_sync;
pub mod native_combat;
pub mod native_session;
pub mod native_vision;
#[cfg(all(test, windows, target_arch = "x86"))]
mod perf_tests;
pub mod persistence;
pub mod protocol;
pub mod registry;
pub mod replication;
mod route_chooser;
#[cfg(test)]
mod send_tests;
pub mod session;
pub mod snapshot;

#[cfg(all(windows, target_arch = "x86"))]
mod native;
#[cfg(all(windows, target_arch = "x86"))]
mod native_access;
#[cfg(all(windows, target_arch = "x86"))]
mod native_call;
#[cfg(all(windows, target_arch = "x86"))]
mod native_gui;
#[cfg(all(windows, target_arch = "x86"))]
mod native_hooks;
#[cfg(all(windows, target_arch = "x86"))]
mod native_journal;
#[cfg(all(windows, target_arch = "x86"))]
mod native_lifecycle;
#[cfg(all(windows, target_arch = "x86"))]
mod native_mp;
#[cfg(all(windows, target_arch = "x86"))]
mod native_route;
#[cfg(all(windows, target_arch = "x86"))]
mod native_signatures;
#[cfg(all(windows, target_arch = "x86"))]
mod native_string;
#[cfg(all(windows, target_arch = "x86"))]
mod patch;
#[cfg(all(windows, target_arch = "x86"))]
mod runtime;
#[cfg(all(windows, target_arch = "x86"))]
mod timing;

#[path = "../../tgcx/src/sha256.rs"]
mod sha256;

#[unsafe(no_mangle)]
pub extern "C" fn tgcx_VolunteersVersion() -> *const std::ffi::c_char {
    c"TGCX Volunteers 0.0.15 development".as_ptr()
}
