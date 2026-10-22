// Turns the headers tools/gen_signatures.py, tgcx_bill_gui.py, tgcx_templates_gui.py and
// tgcx_conform_gui.py generate for the C++ DLL into Rust, so both DLLs build from the same
// generated files. The headers only hold structs, enums, constants and initialiser lists;
// anything else stops the build rather than being skipped.
//
// It also writes the per-button observer stubs (one `push n; jmp common` per button):
// the native glue calls a bare code address, so each button needs its own entry.
use std::collections::HashMap;
use std::fmt::Write as _;
use std::{env, fs, path::Path};

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Ident(String),
    Num(i64),
    Str(Vec<u8>),
    Punct(char),
}

#[derive(Debug, Clone)]
enum Value {
    Num(i64),
    Str(Vec<u8>),
    Bool(bool),
    List(Vec<Value>),
}

#[derive(Debug, Clone)]
enum Ty {
    Str,
    U8,
    I16,
    I32,
    U32,
    Bool,
    Struct(String),
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    ty: Ty,
    dims: Vec<usize>,
}

#[derive(Default)]
struct Header {
    structs: Vec<(String, Vec<Field>)>,
    out: String,
    numbers: HashMap<String, i64>,
    thunks: HashMap<String, usize>,
}

fn tokenize(text: &str, thunks: &mut HashMap<String, usize>) -> Vec<Tok> {
    let mut toks = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix('#') {
            // `#define TGCX_X_THUNKS(M) M(0) M(1) ...`: only the count matters.
            if let Some(def) = rest.trim_start().strip_prefix("define ") {
                let name = def.split('(').next().unwrap().trim().to_string();
                let count = def.matches(" M(").count();
                thunks.insert(name, count);
            }
            continue;
        }
        let b = line.as_bytes();
        let mut i = 0;
        while i < b.len() {
            let c = b[i];
            if c.is_ascii_whitespace() {
                i += 1;
            } else if c == b'/' && b.get(i + 1) == Some(&b'/') {
                break;
            } else if c.is_ascii_alphabetic() || c == b'_' {
                let s = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
                toks.push(Tok::Ident(line[s..i].to_string()));
            } else if c.is_ascii_digit() {
                let s = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric()) {
                    i += 1;
                }
                let raw = line[s..i].trim_end_matches(['u', 'U']);
                let n = match raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
                    Some(hex) => i64::from_str_radix(hex, 16).unwrap(),
                    None => raw.parse().unwrap_or_else(|_| panic!("number {raw}")),
                };
                toks.push(Tok::Num(n));
            } else if c == b'"' {
                i += 1;
                let mut s = Vec::new();
                while b[i] != b'"' {
                    if b[i] == b'\\' {
                        i += 1;
                        match b[i] {
                            b'n' => s.push(b'\n'),
                            b'r' => s.push(b'\r'),
                            b't' => s.push(b'\t'),
                            b'x' => {
                                let mut v = 0u32;
                                i += 1;
                                while i < b.len() && b[i].is_ascii_hexdigit() {
                                    v = v * 16 + (b[i] as char).to_digit(16).unwrap();
                                    i += 1;
                                }
                                s.push(v as u8);
                                continue;
                            }
                            other => s.push(other),
                        }
                        i += 1;
                    } else {
                        s.push(b[i]);
                        i += 1;
                    }
                }
                i += 1;
                // Adjacent literals concatenate.
                if let Some(Tok::Str(prev)) = toks.last_mut() {
                    prev.extend(s);
                } else {
                    toks.push(Tok::Str(s));
                }
            } else {
                toks.push(Tok::Punct(c as char));
                i += 1;
            }
        }
    }
    toks
}

fn rust_name(c: &str) -> String {
    c.strip_prefix("tgcx_").unwrap_or(c).to_ascii_uppercase()
}

fn struct_name(c: &str) -> String {
    c.strip_prefix("tgcx_").unwrap_or(c).to_string()
}

fn enum_name(c: &str) -> String {
    c.strip_prefix("TGCX_").unwrap_or(c).to_string()
}

/// Game text is bytes in the ANSI code page; in Rust each byte is the char of the same
/// number (Latin-1), so `text::bytes` gives the original bytes back.
fn str_lit(s: &[u8]) -> String {
    let mut out = String::from("\"");
    for &c in s {
        match c {
            b'"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            0x20..=0x7E => out.push(c as char),
            _ => write!(out, "\\u{{{c:02X}}}").unwrap(),
        }
    }
    out.push('"');
    out
}

impl Header {
    fn ty(&self, words: &[String], pointer: bool, char_array: &mut bool) -> Ty {
        let words: Vec<&str> = words
            .iter()
            .map(String::as_str)
            .filter(|w| !matches!(*w, "static" | "constexpr" | "const"))
            .collect();
        match (words.as_slice(), pointer) {
            (["char"], true) => Ty::Str,
            (["char"], false) => {
                *char_array = true;
                Ty::Str
            }
            (["unsigned", "char"], false) => Ty::U8,
            (["unsigned"] | ["unsigned", "int"] | ["uint32_t"], false) => Ty::U32,
            (["int"], false) => Ty::I32,
            (["short"], false) => Ty::I16,
            (["bool"], false) => Ty::Bool,
            ([name], false) if self.structs.iter().any(|(n, _)| n == name) => {
                Ty::Struct(name.to_string())
            }
            other => panic!("unsupported C type {other:?}"),
        }
    }

    fn rust_ty(&self, ty: &Ty, dims: &[usize]) -> String {
        let base = match ty {
            Ty::Str => "&'static str".to_string(),
            Ty::U8 => "u8".into(),
            Ty::I16 => "i16".into(),
            Ty::I32 => "i32".into(),
            Ty::U32 => "u32".into(),
            Ty::Bool => "bool".into(),
            Ty::Struct(n) => struct_name(n),
        };
        dims.iter().rev().fold(base, |t, n| format!("[{t}; {n}]"))
    }

    fn zero(&self, ty: &Ty, dims: &[usize]) -> String {
        if let Some((n, rest)) = dims.split_first() {
            return format!("[{}; {n}]", self.zero(ty, rest));
        }
        match ty {
            Ty::Str => "\"\"".into(),
            Ty::Bool => "false".into(),
            Ty::Struct(name) => {
                let fields = &self.structs.iter().find(|(n, _)| n == name).unwrap().1;
                let body: Vec<String> = fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name, self.zero(&f.ty, &f.dims)))
                    .collect();
                format!("{} {{ {} }}", struct_name(name), body.join(", "))
            }
            _ => "0".into(),
        }
    }

    fn value(&self, ty: &Ty, dims: &[usize], v: &Value) -> String {
        if let Some((n, rest)) = dims.split_first() {
            let Value::List(items) = v else {
                panic!("list expected for {ty:?}")
            };
            assert!(items.len() <= *n, "too many initialisers");
            let mut parts: Vec<String> = items.iter().map(|i| self.value(ty, rest, i)).collect();
            while parts.len() < *n {
                parts.push(self.zero(ty, rest));
            }
            return format!("[{}]", parts.join(", "));
        }
        match (ty, v) {
            (Ty::Str, Value::Str(s)) => str_lit(s),
            (Ty::Bool, Value::Bool(b)) => b.to_string(),
            (Ty::Struct(name), Value::List(items)) => {
                let fields = &self.structs.iter().find(|(n, _)| n == name).unwrap().1;
                let body: Vec<String> = fields
                    .iter()
                    .enumerate()
                    .map(|(k, f)| {
                        let value = match items.get(k) {
                            Some(item) => self.value(&f.ty, &f.dims, item),
                            None => self.zero(&f.ty, &f.dims),
                        };
                        format!("{}: {value}", f.name)
                    })
                    .collect();
                format!("{} {{ {} }}", struct_name(name), body.join(", "))
            }
            (Ty::U8 | Ty::I16 | Ty::I32 | Ty::U32, Value::Num(n)) => n.to_string(),
            other => panic!("value {other:?} does not fit"),
        }
    }
}

struct Parser<'a> {
    toks: &'a [Tok],
    at: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.at)
    }
    fn next(&mut self) -> Tok {
        self.at += 1;
        self.toks[self.at - 1].clone()
    }
    fn punct(&mut self, c: char) {
        assert_eq!(self.next(), Tok::Punct(c));
    }
    fn is(&self, c: char) -> bool {
        self.peek() == Some(&Tok::Punct(c))
    }
    fn ident(&mut self) -> String {
        match self.next() {
            Tok::Ident(s) => s,
            t => panic!("identifier expected, got {t:?}"),
        }
    }
    fn value(&mut self, numbers: &HashMap<String, i64>) -> Value {
        match self.next() {
            Tok::Punct('{') => {
                let mut items = Vec::new();
                while !self.is('}') {
                    items.push(self.value(numbers));
                    if self.is(',') {
                        self.at += 1;
                    }
                }
                self.punct('}');
                Value::List(items)
            }
            Tok::Punct('-') => match self.next() {
                Tok::Num(n) => Value::Num(-n),
                t => panic!("number expected, got {t:?}"),
            },
            Tok::Num(n) => Value::Num(n),
            Tok::Str(s) => Value::Str(s),
            Tok::Ident(s) if s == "true" => Value::Bool(true),
            Tok::Ident(s) if s == "false" => Value::Bool(false),
            Tok::Ident(s) => Value::Num(*numbers.get(&s).unwrap_or_else(|| panic!("unknown {s}"))),
            t => panic!("value expected, got {t:?}"),
        }
    }
    /// `[N][]...` after a declarator name; None for `[]`.
    fn dims(&mut self, numbers: &HashMap<String, i64>) -> Vec<Option<usize>> {
        let mut dims = Vec::new();
        while self.is('[') {
            self.at += 1;
            let dim = match self.next() {
                Tok::Punct(']') => {
                    dims.push(None);
                    continue;
                }
                Tok::Num(n) => n as usize,
                Tok::Ident(s) => numbers[&s] as usize,
                t => panic!("dimension expected, got {t:?}"),
            };
            self.punct(']');
            dims.push(Some(dim));
        }
        dims
    }
}

fn parse(text: &str) -> Header {
    let mut h = Header::default();
    let toks = tokenize(text, &mut h.thunks);
    let mut p = Parser { toks: &toks, at: 0 };
    while p.peek().is_some() {
        if p.peek() == Some(&Tok::Ident("struct".into())) {
            p.at += 1;
            let name = p.ident();
            p.punct('{');
            let mut fields = Vec::new();
            while !p.is('}') {
                let mut words = Vec::new();
                let mut pointer = false;
                // Type words, then declarators separated by commas.
                loop {
                    match p.next() {
                        Tok::Ident(w) => words.push(w),
                        Tok::Punct('*') => pointer = true,
                        t => panic!("field type expected, got {t:?}"),
                    }
                    if matches!(p.toks.get(p.at + 1), Some(Tok::Punct('[' | ',' | ';')))
                        && !p.is('*')
                    {
                        break;
                    }
                }
                loop {
                    let fname = p.ident();
                    let dims: Vec<usize> =
                        p.dims(&h.numbers).into_iter().map(Option::unwrap).collect();
                    let mut char_array = false;
                    let ty = h.ty(&words, pointer, &mut char_array);
                    assert!(!char_array, "char arrays in structs are not used");
                    fields.push(Field {
                        name: fname,
                        ty,
                        dims,
                    });
                    if p.is(',') {
                        p.at += 1;
                    } else {
                        break;
                    }
                }
                p.punct(';');
            }
            p.punct('}');
            p.punct(';');
            let mut def = format!(
                "#[derive(Clone, Copy)]\npub struct {} {{\n",
                struct_name(&name)
            );
            h.structs.push((name.clone(), fields.clone()));
            for f in &fields {
                writeln!(def, "    pub {}: {},", f.name, h.rust_ty(&f.ty, &f.dims)).unwrap();
            }
            def.push_str("}\n");
            h.out.push_str(&def);
            continue;
        }
        if p.peek() == Some(&Tok::Ident("enum".into())) {
            p.at += 1;
            let _ = p.ident();
            p.punct('{');
            let mut n = 0;
            while !p.is('}') {
                let name = p.ident();
                writeln!(h.out, "pub const {}: usize = {n};", enum_name(&name)).unwrap();
                h.numbers.insert(name, n);
                n += 1;
                if p.is(',') {
                    p.at += 1;
                }
            }
            p.punct('}');
            p.punct(';');
            continue;
        }
        // A declaration: type words, then `name[dims] = value` declarators.
        let mut words = Vec::new();
        let mut pointer = false;
        while !matches!(
            p.toks.get(p.at + 1),
            Some(Tok::Punct('[' | '=' | ',' | ';'))
        ) {
            match p.next() {
                Tok::Ident(w) => words.push(w),
                Tok::Punct('*') => pointer = true,
                t => panic!("declaration expected, got {t:?}"),
            }
        }
        loop {
            let cname = p.ident();
            let mut dims = p.dims(&h.numbers);
            p.punct('=');
            let value = p.value(&h.numbers);
            let mut char_array = false;
            let ty = h.ty(&words, pointer, &mut char_array);
            if char_array {
                dims.pop();
            }
            // `[]` takes the initialiser's length.
            let mut fixed = Vec::new();
            let mut v = &value;
            for d in &dims {
                let Value::List(items) = v else {
                    panic!("{cname}: list expected")
                };
                fixed.push(d.unwrap_or(items.len()));
                v = items.first().unwrap_or(v);
            }
            if let (Value::Num(n), true) = (&value, dims.is_empty()) {
                h.numbers.insert(cname.clone(), *n);
            }
            let kind = if fixed.is_empty() { "const" } else { "static" };
            let line = format!(
                "pub {kind} {}: {} = {};\n",
                rust_name(&cname),
                h.rust_ty(&ty, &fixed),
                h.value(&ty, &fixed, &value)
            );
            h.out.push_str(&line);
            if p.is(',') {
                p.at += 1;
            } else {
                break;
            }
        }
        p.punct(';');
    }
    h
}

fn generate(header: &str, out: &Path) -> Header {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../dll/tgcx")
        .join(header);
    println!("cargo:rerun-if-changed={}", path.display());
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let h = parse(&text);
    let mut file = format!("// Generated by build.rs from dll/tgcx/{header}.\n");
    file.push_str(&h.out);
    fs::write(out.join(header.replace(".h", ".rs")), file).unwrap();
    h
}

/// One `push n; jmp common` stub per button and a table of their addresses.
fn stubs(asm: &mut String, prefix: &str, count: usize) {
    writeln!(asm, "    \".p2align 4\",").unwrap();
    for n in 0..count {
        writeln!(
            asm,
            "    \"tgcx_{prefix}_stub_{n}:\", \"push {n}\", \"jmp {{{prefix}}}\","
        )
        .unwrap();
    }
    writeln!(asm, "    \".section .rdata,\\\"dr\\\"\", \".p2align 2\",").unwrap();
    writeln!(
        asm,
        "    \".globl _tgcx_{prefix}_table\", \"_tgcx_{prefix}_table:\","
    )
    .unwrap();
    for n in 0..count {
        writeln!(asm, "    \".long tgcx_{prefix}_stub_{n}\",").unwrap();
    }
    writeln!(asm, "    \".text\",").unwrap();
}

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    let out = Path::new(&out);
    generate("tgcx_signatures.h", out);
    let bill = generate("tgcx_bill_catalog.h", out);
    let tpl = generate("tgcx_templates_catalog.h", out);
    let cf = generate("tgcx_conform_layout.h", out);

    // Bill: every window button plus Politics' Open Bill.
    let bill_count = bill.numbers["TGCX_B_COUNT_OF"] as usize + 1;
    let tpl_count = tpl.thunks["TGCX_TPL_THUNKS"];
    let cf_count = cf.thunks["TGCX_CF_THUNKS"];
    assert_eq!(
        tpl_count, tpl.numbers["TGCX_TPL_BUTTONS"] as usize,
        "one stub per Templates button"
    );
    assert_eq!(
        cf_count, cf.numbers["TGCX_CF_BUTTONS"] as usize,
        "one stub per Conform button"
    );

    let mut asm = String::from("core::arch::global_asm!(\n");
    stubs(&mut asm, "bill", bill_count);
    stubs(&mut asm, "tpl", tpl_count);
    stubs(&mut asm, "cf", cf_count);
    asm.push_str("    bill = sym crate::bill::bill_thunk_common,\n");
    asm.push_str("    tpl = sym crate::templates_gui::tpl_thunk_common,\n");
    asm.push_str("    cf = sym crate::conform_gui::cf_thunk_common,\n);\n");
    writeln!(asm, "pub const BILL_THUNKS: usize = {bill_count};").unwrap();
    writeln!(asm, "pub const TPL_THUNKS: usize = {tpl_count};").unwrap();
    writeln!(asm, "pub const CF_THUNKS: usize = {cf_count};").unwrap();
    asm.push_str(
        "unsafe extern \"C\" {\n    pub static tgcx_bill_table: [usize; BILL_THUNKS];\n    \
         pub static tgcx_tpl_table: [usize; TPL_THUNKS];\n    pub static tgcx_cf_table: [usize; CF_THUNKS];\n}\n",
    );
    fs::write(out.join("thunks.rs"), asm).unwrap();
}
