// Turns the 128 `#pragma comment(linker, "/export:NAME=lua51_real.NAME")` lines of
// dll/tgcx/tgcx_exports.h into a .drectve section, so the C++ and Rust loaders share
// one list. link.exe accepts such forwarders only as directives in an object: as a
// command-line /EXPORT or in a .def file they fail with LNK2001.
use std::{env, fs, path::Path};

const PRAGMA: &str = "#pragma comment(linker, \"/export:";

fn main() {
    let header = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dll/tgcx/tgcx_exports.h");
    println!("cargo:rerun-if-changed={}", header.display());
    let text = fs::read_to_string(&header).expect("read dll/tgcx/tgcx_exports.h");

    let exports: Vec<&str> = text
        .lines()
        .filter_map(|line| line.trim().strip_prefix(PRAGMA)?.strip_suffix("\")"))
        .collect();
    assert_eq!(exports.len(), 128, "the original lua51.dll has 128 exports");

    let directives: String = exports.iter().map(|e| format!(" /EXPORT:{e}")).collect();
    let directives = directives + " ";
    let len = directives.len();
    let source = format!(
        "#[used]\n#[unsafe(link_section = \".drectve\")]\n\
         static LUA_FORWARDS: [u8; {len}] = *b\"{directives}\";\n"
    );
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("forwards.rs");
    fs::write(out, source).unwrap();
}
