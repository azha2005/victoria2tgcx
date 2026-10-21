//! Port of the part of dll/tgcx/tgcx_packages.h the loader needs (MOD-2, D15): which
//! of our packages the launcher ticked. The launcher passes one -mod=<file>.mod per
//! ticked mod; each package folder holds tgcx_package.txt and its own tgcx.dll.

pub const MAX_PATH: usize = 260;
pub const MODS_MAX: usize = 24;
/// Size of tgcx_Mod::package, NUL included.
pub const PACKAGE_SIZE: usize = 16;

/// Every -mod= value, in command-line order. A value runs to its ".mod", so a file
/// name may hold spaces whether or not the launcher quotes it.
pub fn parse_mod_args(line: &[u8]) -> Vec<&[u8]> {
    let line = until_nul(line);
    let mut mods = Vec::new();
    let mut at = 0;
    while mods.len() < MODS_MAX {
        let Some(found) = line[at..].windows(5).position(|w| w == b"-mod=") else {
            break;
        };
        let start = at + found + 5;
        let Some(end) = (start..line.len()).find(|&i| ends_mod(line, i)) else {
            break;
        };
        at = end + 4;
        let value = &line[start..at];
        let value = value.strip_prefix(b"\"").unwrap_or(value);
        if !value.is_empty() && value.len() < MAX_PATH {
            mods.push(value);
        }
    }
    mods
}

/// ".mod" (any case) at `i`, followed by the end of the line, a space or a quote.
fn ends_mod(line: &[u8], i: usize) -> bool {
    line.get(i..i + 4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b".mod"))
        && matches!(line.get(i + 4), None | Some(b' ' | b'"'))
}

/// The value of the first `key = "value"` (or unquoted) line. The key must start the
/// line, so "path" never matches "replace_path". Like the C++, None when the value
/// and its NUL do not fit in `size` bytes.
pub fn mod_value<'a>(text: &'a [u8], key: &str, size: usize) -> Option<&'a [u8]> {
    for line in until_nul(text).split(|&c| c == b'\n') {
        let Some((name, rest)) = skip_blanks(line).split_at_checked(key.len()) else {
            continue;
        };
        if !name.eq_ignore_ascii_case(key.as_bytes())
            || !matches!(rest.first(), Some(b' ' | b'\t' | b'='))
        {
            continue;
        }
        let Some(value) = skip_blanks(rest).strip_prefix(b"=") else {
            continue;
        };
        let value = skip_blanks(value);
        let (value, quoted) = match value.strip_prefix(b"\"") {
            Some(value) => (value, true),
            None => (value, false),
        };
        let ends = |c: u8| c == b'\r' || if quoted { c == b'"' } else { is_blank(c) };
        let len = value.iter().position(|&c| ends(c)).unwrap_or(value.len());
        return (len < size).then(|| &value[..len]);
    }
    None
}

fn is_blank(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn skip_blanks(s: &[u8]) -> &[u8] {
    &s[s.iter().take_while(|&&c| is_blank(c)).count()..]
}

fn until_nul(s: &[u8]) -> &[u8] {
    s.split(|&c| c == 0).next().unwrap_or_default()
}
