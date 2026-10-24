//! TPL-11 Build Army gathering (D20), pure: nothing here reads or writes the game. One
//! Recruit click makes one template army. Each regiment is built in its own source
//! province, then marches by a safe land path and merges into that click's army, the
//! first regiment to appear becoming the army (conform_native's daily driver). The
//! target is the best-supply province (or the capital, or the land rally point); if one of our armies already stands there, or an earlier click still gathers
//! there, the army gathers in the next free province around it. Native reads:
//! docs/symbols.md "TPL-11".

use crate::conform::{CfId, id_pair};

pub const MODE_BEST: u32 = 0;
pub const MODE_CAPITAL: u32 = 1;
/// The first land rally point; the k-th is `MODE_RALLY + k`.
pub const MODE_RALLY: u32 = 2;

/// The footer's arrows cycle Best supply, Capital, then each land rally point (one
/// "Rally point" entry when there is none).
pub fn mode_step(mode: u32, forward: bool, rallies: u32) -> u32 {
    let modes = MODE_RALLY + rallies.max(1);
    // a mode past the end (a rally point removed since) counts as the last one
    let mode = mode.min(modes - 1);
    if forward {
        (mode + 1) % modes
    } else {
        (mode + modes - 1) % modes
    }
}

/// An owned and controlled land province and its supply limit (thousandths).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Site {
    pub province: i32,
    pub supply: i32,
}

/// Highest supply first, then the lower province id, so the choice is stable.
fn by_supply(a: &Site, b: &Site) -> std::cmp::Ordering {
    b.supply.cmp(&a.supply).then(a.province.cmp(&b.province))
}

/// Best supply: the first site, by supply, that all `total` planned regiments can reach
/// (`reach` counts how many can). When none can, the site most of them reach; None
/// when no regiment reaches any. Only the best `tries` sites are asked: each answer
/// costs route queries.
pub fn best(
    sites: &[Site],
    total: u32,
    tries: usize,
    reach: &mut dyn FnMut(i32) -> u32,
) -> Option<i32> {
    let mut sorted = sites.to_vec();
    sorted.sort_by(by_supply);
    let mut fallback: Option<(u32, i32)> = None;
    for site in sorted.iter().take(tries) {
        let n = reach(site.province);
        if n >= total && total != 0 {
            return Some(site.province);
        }
        if n != 0 && fallback.is_none_or(|(m, _)| n > m) {
            fallback = Some((n, site.province));
        }
    }
    fallback.map(|(_, province)| province)
}

/// How far around a taken target the army may gather, in rings of neighbours.
pub const RINGS: usize = 6;

/// The base if it is free; otherwise the free province nearest to it, ring by ring
/// through land neighbours we own and control (`supply` is None for any other), the
/// highest supply within the first ring that has one. None when every province within
/// RINGS is taken.
pub fn around(
    base: i32,
    neighbours: &mut dyn FnMut(i32) -> Vec<i32>,
    supply: &mut dyn FnMut(i32) -> Option<i32>,
    taken: &dyn Fn(i32) -> bool,
) -> Option<i32> {
    if !taken(base) {
        return Some(base);
    }
    let mut seen = vec![base];
    let mut ring = vec![base];
    for _ in 0..RINGS {
        let mut next = Vec::new();
        for &p in &ring {
            for n in neighbours(p) {
                if seen.contains(&n) {
                    continue;
                }
                seen.push(n);
                if let Some(s) = supply(n) {
                    next.push(Site {
                        province: n,
                        supply: s,
                    });
                }
            }
        }
        next.sort_by(by_supply);
        if let Some(free) = next.iter().find(|s| !taken(s.province)) {
            return Some(free.province);
        }
        if next.is_empty() {
            return None;
        }
        ring = next.iter().map(|s| s.province).collect();
    }
    None
}

// ---------------------------------------------------------------- saved orders (as TPL-7)

// One line per regiment being built for a gathering army, in its own local file beside
// the templates (tgcx_gather_orders.txt), so a 0.11.3 build never meets a line it does
// not know in its Conform orders file. `group` names the click; `army` is the stack
// that became its army (0/0 until a regiment appears). Same date window as the Conform
// orders: a save made before the click, or after the order ended, never resumes it.
pub const GATHER_MAX: usize = 512;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct GtOrder {
    pub tag: u32,
    pub created: u32,
    pub done: u32,
    pub province: i32,
    pub group: u32,
    pub army: CfId,
    pub stack: CfId,
    pub construction: CfId,
    pub unit: i32,
    pub active: bool,
}

pub fn gather_applies(r: &GtOrder, tag: u32, date: u32) -> bool {
    r.tag == tag && r.created <= date && (r.done == 0 || date < r.done)
}

/// Full: the oldest ended order goes first, else the oldest open one. Returns the new
/// order's index and the index removed (later ones moved down one), if any.
pub fn gather_add(k: &mut Vec<GtOrder>, r: GtOrder) -> (usize, Option<usize>) {
    let mut dropped = None;
    if k.len() >= GATHER_MAX {
        let (mut drop, mut done) = (0, false);
        for (i, c) in k.iter().enumerate() {
            let d = c.done != 0;
            if (d && !done) || (d == done && c.created < k[drop].created) {
                drop = i;
                done = d;
            }
        }
        k.remove(drop);
        dropped = Some(drop);
    }
    k.push(GtOrder { active: false, ..r });
    (k.len() - 1, dropped)
}

/// A group id no saved order uses yet.
pub fn next_group(k: &[GtOrder], live: &[u32]) -> u32 {
    k.iter()
        .map(|r| r.group)
        .chain(live.iter().copied())
        .max()
        .unwrap_or(0)
        .wrapping_add(1)
        .max(1)
}

fn gather_line(r: &GtOrder) -> String {
    format!(
        "gather tag={:08X} created={} done={} province={} group={} army={}/{} stack={}/{} construction={}/{} unit={}\r\n",
        r.tag,
        r.created,
        r.done,
        r.province,
        r.group,
        r.army.kind,
        r.army.id,
        r.stack.kind,
        r.stack.id,
        r.construction.kind,
        r.construction.id,
        r.unit
    )
}

const GATHER_HEADER: &str = "tgcx_gather_orders 1\r\n";

/// The file's text, or None when it would not fit in `size` bytes.
pub fn gather_write(k: &[GtOrder], size: usize) -> Option<String> {
    let mut out = String::from(GATHER_HEADER);
    for r in k {
        out.push_str(&gather_line(r));
    }
    (out.len() < size).then_some(out)
}

fn parse_gather(line: &str) -> Option<GtOrder> {
    let mut r = GtOrder::default();
    let rest = line.strip_prefix("gather tag=")?;
    let (tag, rest) = rest.split_once(" created=")?;
    r.tag = u32::from_str_radix(tag, 16).ok()?;
    let (created, rest) = rest.split_once(" done=")?;
    r.created = created.parse().ok()?;
    let (done, rest) = rest.split_once(" province=")?;
    r.done = done.parse().ok()?;
    let (province, rest) = rest.split_once(" group=")?;
    r.province = province.parse().ok()?;
    let (group, rest) = rest.split_once(" army=")?;
    r.group = group.parse().ok()?;
    let (army, rest) = rest.split_once(" stack=")?;
    r.army = id_pair(army)?;
    let (stack, rest) = rest.split_once(" construction=")?;
    r.stack = id_pair(stack)?;
    let (construction, unit) = rest.split_once(" unit=")?;
    r.construction = id_pair(construction)?;
    r.unit = unit.parse().ok()?;
    Some(r)
}

/// Strict, as the Conform orders: every line must print back byte for byte, or the
/// whole file is refused (nothing is resumed from a damaged one).
pub fn gather_parse(text: &[u8], units: usize) -> Option<Vec<GtOrder>> {
    let text = text.split(|&c| c == 0).next().unwrap_or_default();
    let mut at = text.strip_prefix(GATHER_HEADER.as_bytes())?;
    let mut out = Vec::new();
    while !at.is_empty() {
        let end = at.windows(2).position(|w| w == b"\r\n")?;
        if end >= 230 || out.len() >= GATHER_MAX {
            return None;
        }
        let r = parse_gather(std::str::from_utf8(&at[..end]).ok()?)?;
        if gather_line(&r).as_bytes() != &at[..end + 2]
            || r.group == 0
            || r.stack.none()
            || !(0..=65535).contains(&r.province)
            || !(0..units as i32).contains(&r.unit)
        {
            return None;
        }
        out.push(r);
        at = &at[end + 2..];
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site(province: i32, supply: i32) -> Site {
        Site { province, supply }
    }

    #[test]
    fn modes_cycle_both_ways() {
        assert_eq!(mode_step(MODE_BEST, true, 0), MODE_CAPITAL);
        assert_eq!(mode_step(MODE_RALLY, true, 0), MODE_BEST);
        assert_eq!(mode_step(MODE_BEST, false, 0), MODE_RALLY);
        assert_eq!(mode_step(7, true, 0), MODE_BEST); // out of range counts as the last
        // three rally points: Best, Capital, R1, R2, R3
        assert_eq!(mode_step(MODE_RALLY, true, 3), MODE_RALLY + 1);
        assert_eq!(mode_step(MODE_RALLY + 2, true, 3), MODE_BEST);
        assert_eq!(mode_step(MODE_BEST, false, 3), MODE_RALLY + 2);
        // a rally point removed since: the stale mode counts as the last one
        assert_eq!(mode_step(MODE_RALLY + 2, true, 1), MODE_BEST);
    }

    #[test]
    fn best_takes_the_highest_supply_every_source_reaches() {
        let sites = [
            site(10, 9000),
            site(11, 20000),
            site(12, 20000),
            site(13, 5000),
        ];
        // 11 and 12 tie on supply: the lower id is asked first. 11 is an island
        // only 2 of 5 regiments reach; 12 is reached by all.
        let mut asked = Vec::new();
        let mut reach = |p: i32| {
            asked.push(p);
            match p {
                11 => 2,
                _ => 5,
            }
        };
        assert_eq!(best(&sites, 5, 8, &mut reach), Some(12));
        assert_eq!(asked, vec![11, 12]);
    }

    #[test]
    fn best_falls_back_to_the_most_reached_site() {
        let sites = [site(1, 30000), site(2, 20000), site(3, 10000)];
        let mut reach = |p: i32| match p {
            1 => 1,
            2 => 3,
            _ => 3,
        };
        // Nothing reaches all 4: the site most regiments reach, by supply on a tie.
        assert_eq!(best(&sites, 4, 8, &mut reach), Some(2));
        let mut none = |_| 0;
        assert_eq!(best(&sites, 4, 8, &mut none), None);
        // Only `tries` sites are asked.
        let mut late = |p: i32| if p == 3 { 4 } else { 0 };
        assert_eq!(best(&sites, 4, 2, &mut late), None);
    }

    /// A line of provinces 1-2-3-4-5, plus 6 beside 1; 9 is foreign (no supply).
    fn map(p: i32) -> Vec<i32> {
        match p {
            1 => vec![2, 6, 9],
            2 => vec![1, 3],
            3 => vec![2, 4],
            4 => vec![3, 5],
            5 => vec![4],
            6 => vec![1],
            9 => vec![1],
            _ => vec![],
        }
    }

    fn supply(p: i32) -> Option<i32> {
        match p {
            9 => None,
            6 => Some(4000),
            p => Some(p * 1000),
        }
    }

    #[test]
    fn around_keeps_a_free_base_and_fills_rings_by_supply() {
        let none = |_| false;
        assert_eq!(around(1, &mut map, &mut supply, &none), Some(1));
        // 1 taken: ring one is 2 (2000) and 6 (4000); 9 is not ours.
        let one = |p| p == 1;
        assert_eq!(around(1, &mut map, &mut supply, &one), Some(6));
        let more = |p| p == 1 || p == 6;
        assert_eq!(around(1, &mut map, &mut supply, &more), Some(2));
        // Ring one full: ring two.
        let ring = |p| p == 1 || p == 6 || p == 2;
        assert_eq!(around(1, &mut map, &mut supply, &ring), Some(3));
        let all = |p| p != 9;
        assert_eq!(around(1, &mut map, &mut supply, &all), None);
    }

    #[test]
    fn around_never_walks_through_foreign_provinces() {
        // 9 leads nowhere of ours: from 9 only 1 is a neighbour, already seen.
        let mut island = |p: i32| if p == 7 { vec![9] } else { map(p) };
        let taken = |p| p == 7;
        assert_eq!(around(7, &mut island, &mut supply, &taken), None);
    }

    fn order(group: u32, unit: i32) -> GtOrder {
        GtOrder {
            tag: 0x00545541,
            created: 1000,
            done: 0,
            province: 641,
            group,
            army: CfId::default(),
            stack: CfId { kind: 4713, id: 77 },
            construction: CfId { kind: 3, id: 9 },
            unit,
            active: true,
        }
    }

    #[test]
    fn orders_round_trip_and_refuse_damage() {
        let mut k = vec![order(1, 3), order(1, 5)];
        k[1].army = CfId { kind: 4713, id: 76 };
        k[1].done = 1200;
        let text = gather_write(&k, 4096).unwrap();
        assert!(text.starts_with("tgcx_gather_orders 1\r\ngather tag=00545541 created=1000"));
        let back = gather_parse(text.as_bytes(), 32).unwrap();
        assert_eq!(back.len(), 2);
        assert_eq!(back[1].army, CfId { kind: 4713, id: 76 });
        assert!(!back[0].active); // activity is this session's, never saved
        assert!(gather_parse(text.replace("unit=5", "unit=05").as_bytes(), 32).is_none());
        assert!(gather_parse(text.replace("group=1 ", "group=0 ").as_bytes(), 32).is_none());
        assert!(gather_parse(text.as_bytes(), 4).is_none()); // unit index out of range
        // A Conform orders file is not a gather file.
        assert!(gather_parse(b"tgcx_conform_orders 1\r\n", 32).is_none());
        assert_eq!(
            gather_parse(b"tgcx_gather_orders 1\r\n", 32),
            Some(Vec::new())
        );
        assert!(gather_write(&k, 64).is_none());
    }

    #[test]
    fn orders_window_eviction_and_group_ids() {
        let r = order(4, 3);
        assert!(gather_applies(&r, 0x00545541, 1000));
        assert!(!gather_applies(&r, 0x00545541, 999));
        assert!(!gather_applies(&r, 0x00415355, 1000));
        let ended = GtOrder { done: 1100, ..r };
        assert!(!gather_applies(&ended, 0x00545541, 1100));
        let mut k = vec![r; GATHER_MAX];
        k[7].done = 1100;
        let (index, dropped) = gather_add(&mut k, order(5, 3));
        assert_eq!((index, dropped), (GATHER_MAX - 1, Some(7)));
        assert_eq!(next_group(&k, &[]), 6);
        assert_eq!(next_group(&k, &[9]), 10);
        assert_eq!(next_group(&[], &[]), 1);
    }
}
