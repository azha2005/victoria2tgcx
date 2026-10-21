// The 128 Lua forwards come from dll/tgcx/tgcx_exports.h (from V2DLL: lua51_exports.h,
// see R8), so the C++ and Rust loaders share one list. link.exe only accepts
// `NAME=lua51_real.NAME` forwarders as linker directives in an object (what the
// header's #pragma comment(linker, ...) emits); as a command-line /EXPORT or in a
// .def file they fail with LNK2001. So the same directives go into a .drectve
// section (src/lib.rs includes the generated static).
use std::{env, fs, path::Path};

fn main() {
    let header = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../dll/tgcx/tgcx_exports.h");
    println!("cargo:rerun-if-changed={}", header.display());
    let text = fs::read_to_string(&header).expect("dll/tgcx/tgcx_exports.h");
    let directives: Vec<String> = text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("#pragma comment(linker, \"/export:"))
        .map(|rest| format!(" /EXPORT:{}", rest.strip_suffix("\")").expect("export pragma ends with \")")))
        .collect();
    assert_eq!(directives.len(), 128, "the original lua51.dll has 128 exports");
    let bytes = directives.concat() + " ";
    let source = format!(
        "#[used]\n#[unsafe(link_section = \".drectve\")]\nstatic LUA_FORWARDS: [u8; {}] = *b\"{}\";\n",
        bytes.len(),
        bytes
    );
    fs::write(Path::new(&env::var("OUT_DIR").unwrap()).join("forwards.rs"), source).unwrap();
}
