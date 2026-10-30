//! Read-only native scene-order check. Do not raise above later dialogs on polls.
//! Render links +D4/+D8/+E4 and window+494 follow the existing native GUI evidence.
use crate::snapshot::Memory;
fn field(m: &impl Memory, p: u32, offset: u32) -> Option<u32> {
    m.read32(p.checked_add(offset)?)
}
fn branch(m: &impl Memory, mut inner: u32, scene: u32) -> Option<u32> {
    let mut seen = [0; 8];
    for i in 0..seen.len() {
        if inner == 0 || seen[..i].contains(&inner) {
            return None;
        }
        seen[i] = inner;
        let owner = field(m, inner, 0xE4)?;
        if owner == scene {
            return Some(inner);
        }
        inner = owner;
    }
    None
}
/// Repair only when the native host is last and demonstrably covers our entry.
/// A later dialog keeps its order; after it closes the next update can repair.
pub fn covered_by_host(m: &impl Memory, window_vtable: u32, root: u32, entry: u32) -> Option<bool> {
    let renderer = field(m, root, 0x90)?;
    if field(m, entry, 0x90)? != renderer {
        return None;
    }
    let scene = renderer.checked_add(0x6B9E0)?;
    let host = branch(m, field(m, root, 0x494)?, scene)?;
    let button = branch(m, field(m, entry, 0x494)?, scene)?;
    let mut tail = field(m, scene, 0xE0)?;
    // Hidden, recognized window renderers can remain linked after a dialog
    // closes. Unknown/visible later roots retain their foreground order.
    for _ in 0..256 {
        if tail == host {
            break;
        }
        if field(m, tail, 0xE4)? != scene {
            return None;
        }
        if m.read32(tail)? != window_vtable || field(m, tail, 0x124)? as u8 != 0 {
            return Some(false);
        }
        let previous = field(m, tail, 0xD4)?;
        if previous == 0 || field(m, previous, 0xD8)? != tail {
            return None;
        }
        tail = previous;
    }
    if tail != host {
        return None;
    }
    if host == button {
        return Some(true);
    }
    let mut node = host;
    let mut seen = [0; 256];
    for i in 0..seen.len() {
        if node == 0 || seen[..i].contains(&node) || field(m, node, 0xE4)? != scene {
            return None;
        }
        if node == button {
            return Some(true);
        }
        seen[i] = node;
        let previous = field(m, node, 0xD4)?;
        if previous == 0 || field(m, previous, 0xD8)? != node {
            return None;
        }
        node = previous;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    struct Fixture(BTreeMap<u32, u32>);
    impl Memory for Fixture {
        fn read32(&self, p: u32) -> Option<u32> {
            self.0.get(&p).copied()
        }
    }
    fn scene(order: &[u32]) -> Fixture {
        let mut m = BTreeMap::from([
            (0x10090, 0x50000),
            (0x20090, 0x50000),
            (0x10494, 0x30000),
            (0x20494, 0x40000),
            (0xBB9E0 + 0xE0, *order.last().unwrap()),
        ]);
        for (i, &node) in order.iter().enumerate() {
            m.insert(node, 42);
            m.insert(node + 0x124, 1);
            m.insert(node + 0xE4, 0xBB9E0);
            m.insert(node + 0xD4, if i == 0 { 0 } else { order[i - 1] });
            m.insert(node + 0xD8, order.get(i + 1).copied().unwrap_or(0));
        }
        Fixture(m)
    }
    #[test]
    fn host_reorder_recovers_once_without_raising_over_a_later_dialog() {
        assert_eq!(
            covered_by_host(&scene(&[0x40000, 0x30000]), 42, 0x10000, 0x20000),
            Some(true)
        );
        assert_eq!(
            covered_by_host(&scene(&[0x30000, 0x40000]), 42, 0x10000, 0x20000),
            Some(false)
        );
        assert_eq!(
            covered_by_host(&scene(&[0x30000, 0x40000, 0x60000]), 42, 0x10000, 0x20000),
            Some(false)
        );
        assert_eq!(
            covered_by_host(&scene(&[0x40000, 0x30000, 0x60000]), 42, 0x10000, 0x20000),
            Some(false)
        );
    }
    #[test]
    fn cycles_torn_links_and_wrong_renderers_never_request_promotion() {
        let mut m = scene(&[0x40000, 0x30000]);
        m.0.insert(0x40000 + 0xD8, 0);
        assert_eq!(covered_by_host(&m, 42, 0x10000, 0x20000), None);
        let mut m = scene(&[0x40000, 0x30000]);
        m.0.insert(0x30000 + 0xD4, 0x30000);
        assert_eq!(covered_by_host(&m, 42, 0x10000, 0x20000), None);
        let mut m = scene(&[0x40000, 0x30000]);
        m.0.insert(0x20090, 0x70000);
        assert_eq!(covered_by_host(&m, 42, 0x10000, 0x20000), None);
    }
    #[test]
    fn a_closed_dialog_does_not_block_entry_recovery_forever() {
        let mut m = scene(&[0x40000, 0x30000, 0x60000]);
        m.0.insert(0x60000 + 0x124, 0);
        assert_eq!(covered_by_host(&m, 42, 0x10000, 0x20000), Some(true));
        m.0.insert(0x60000, 43);
        assert_eq!(covered_by_host(&m, 42, 0x10000, 0x20000), Some(false));
    }
}
