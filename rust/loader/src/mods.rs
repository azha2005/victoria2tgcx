//! Port of dll/tgcx/tgcx_packages.h, the part the loader needs (MOD-2, D15): which
//! of our packages the launcher ticked. The launcher passes one -mod=<file>.mod
//! per ticked mod; each package folder holds tgcx_package.txt and its own tgcx.dll.

pub const MAX_PATH: usize = 260;
pub const MODS_MAX: usize = 24;
// Field sizes of tgcx_Mod, NUL included: a longer value is not read at all.
pub const PATH_SIZE: usize = MAX_PATH;
pub const PACKAGE_SIZE: usize = 16;

/// Every -mod= value, in command-line order. A value runs to its ".mod", so a
/// file name may hold spaces whether or not the launcher quotes it.
pub fn parse_mod_args(line: &[u8]) -> Vec<&[u8]> {
    let line = until_nul(line);
    let mut out = Vec::new();
    let mut at = 0;
    while out.len() < MODS_MAX {
        let Some(found) = find(&line[at..], b"-mod=") else { break };
        let start = at + found + 5;
        let Some(end) = (start..line.len()).find(|&i| ends_mod(line, i)) else { break };
        let end = end + 4;
        let value = &line[start..end];
        let value = value.strip_prefix(b"\"").unwrap_or(value);
        if !value.is_empty() && value.len() < MAX_PATH {
            out.push(value);
        }
        at = end;
    }
    out
}

/// ".mod" (any case) at `i`, followed by the end of the line, a space or a quote.
fn ends_mod(line: &[u8], i: usize) -> bool {
    line.len() >= i + 4
        && line[i..i + 4].eq_ignore_ascii_case(b".mod")
        && matches!(line.get(i + 4), None | Some(b' ') | Some(b'"'))
}

/// The value of the first `key = "value"` (or unquoted) line. The key must start
/// the line, so "path" never matches "replace_path". None when the value does not
/// fit `size` bytes with its NUL, as the C++ does.
pub fn mod_value<'a>(text: &'a [u8], key: &str, size: usize) -> Option<&'a [u8]> {
    let key = key.as_bytes();
    for line in until_nul(text).split(|&c| c == b'\n') {
        let p = skip_blanks(line);
        let Some(after) = p.get(..key.len()).filter(|k| k.eq_ignore_ascii_case(key)).map(|_| &p[key.len()..]) else {
            continue;
        };
        if !matches!(after.first(), Some(b' ' | b'\t' | b'=')) {
            continue;
        }
        let Some(value) = skip_blanks(after).strip_prefix(b"=") else { continue };
        let value = skip_blanks(value);
        let (value, quoted) = match value.strip_prefix(b"\"") {
            Some(v) => (v, true),
            None => (value, false),
        };
        let n = value
            .iter()
            .position(|&c| c == b'\r' || if quoted { c == b'"' } else { c == b' ' || c == b'\t' })
            .unwrap_or(value.len());
        return (n < size).then(|| &value[..n]);
    }
    None
}

fn skip_blanks(s: &[u8]) -> &[u8] {
    let n = s.iter().take_while(|&&c| c == b' ' || c == b'\t').count();
    &s[n..]
}

fn until_nul(s: &[u8]) -> &[u8] {
    &s[..s.iter().position(|&c| c == 0).unwrap_or(s.len())]
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_args_in_order() {
        let line = br#""C:\game\v2game.exe" -mod=mod/TGC.mod -mod=mod/tgcx_army_universal.mod"#;
        assert_eq!(parse_mod_args(line), [&b"mod/TGC.mod"[..], b"mod/tgcx_army_universal.mod"]);
    }

    #[test]
    fn mod_args_with_spaces_and_quotes() {
        let line = br#"v2game.exe -mod="mod/My Mod.MOD" -mod=mod/Other Mod.mod"#;
        assert_eq!(parse_mod_args(line), [&b"mod/My Mod.MOD"[..], b"mod/Other Mod.mod"]);
    }

    #[test]
    fn mod_args_need_a_dot_mod_end() {
        assert!(parse_mod_args(b"v2game.exe -mod=mod/a.modx").is_empty());
        assert!(parse_mod_args(b"v2game.exe -mod=mod/a.txt").is_empty());
        assert!(parse_mod_args(b"v2game.exe").is_empty());
    }

    #[test]
    fn mod_args_capped() {
        let line: Vec<u8> = (0..30).flat_map(|i| format!(" -mod=mod/m{i}.mod").into_bytes()).collect();
        let mods = parse_mod_args(&line);
        assert_eq!(mods.len(), MODS_MAX);
        assert_eq!(mods[23], b"mod/m23.mod");
    }

    #[test]
    fn mod_args_too_long_are_skipped() {
        let long = format!("-mod=mod/{}.mod -mod=mod/b.mod", "x".repeat(MAX_PATH));
        assert_eq!(parse_mod_args(long.as_bytes()), [&b"mod/b.mod"[..]]);
    }

    #[test]
    fn values() {
        let text = b"name = \"ATCU\"\r\nreplace_path = \"x\"\r\n\tPATH=mod/tgcx_army_universal\r\nuser_dir = \"TGC252\"\r\n";
        assert_eq!(mod_value(text, "path", PATH_SIZE), Some(&b"mod/tgcx_army_universal"[..]));
        assert_eq!(mod_value(text, "user_dir", 64), Some(&b"TGC252"[..]));
        assert_eq!(mod_value(text, "name", 64), Some(&b"ATCU"[..]));
        assert_eq!(mod_value(text, "package", PACKAGE_SIZE), None);
    }

    #[test]
    fn value_key_must_be_whole_and_assigned() {
        assert_eq!(mod_value(b"paths = a\npath b\npath = c\n", "path", PATH_SIZE), Some(&b"c"[..]));
    }

    #[test]
    fn value_too_long_is_none() {
        assert_eq!(mod_value(b"package = army_and_more_than_16\n", "package", PACKAGE_SIZE), None);
        assert_eq!(mod_value(b"package = army\n", "package", PACKAGE_SIZE), Some(&b"army"[..]));
    }

    #[test]
    fn value_stops_at_nul() {
        assert_eq!(mod_value(b"a = 1\0\npackage = army\n", "package", PACKAGE_SIZE), None);
    }
}
