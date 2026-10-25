//! TPL-12 Replace depleted (mockup 6-conform-replace-choose-1024), pure: nothing here
//! reads or writes the game. The chooser beside Conform picks what counts as depleted:
//! low strength (below a threshold, 10% to 90% in steps of 10) and/or a pop too small
//! (the unit panel's red or yellow). Each depleted regiment gets one new regiment of
//! its type, built by Conform's recruit (culture, land reach); when that one has
//! merged, the old one is detached into its own stack (Create + Transfers, P51) for
//! the player to keep or disband. Native reads: docs/symbols.md "TPL-12".

use crate::conform::{CfId, id_pair};
use crate::templates::{CULTURE_ALL, CULTURE_NONCORE};

pub const BELOW_MIN: u32 = 10;
pub const BELOW_MAX: u32 = 90;
pub const BELOW_STEP: u32 = 10;

/// What the chooser counts as depleted (user, 2026-10-24: both by default, 50%).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Choice {
    pub strength: bool,
    pub pop: bool,
    /// Percent of the type's full strength.
    pub below: u32,
    /// Independent of the selected Conform template; none means all cultures.
    pub culture: u32,
}

impl Choice {
    pub const DEFAULT: Choice = Choice {
        strength: true,
        pop: true,
        below: 50,
        culture: CULTURE_ALL,
    };

    /// Slider step 0..=8.
    pub fn step(&self) -> u32 {
        (self.below - BELOW_MIN) / BELOW_STEP
    }

    pub fn set_step(&mut self, step: u32) {
        self.below = (BELOW_MIN + step * BELOW_STEP).min(BELOW_MAX);
    }

    pub fn nudge(&mut self, up: bool) {
        self.below = if up {
            (self.below + BELOW_STEP).min(BELOW_MAX)
        } else {
            self.below.saturating_sub(BELOW_STEP).max(BELOW_MIN)
        };
    }

    pub fn toggle_culture(&mut self, mode: u32) {
        if mode <= CULTURE_NONCORE {
            self.culture = if self.culture == mode {
                CULTURE_ALL
            } else {
                mode
            };
        }
    }
}

/// One regiment as the native layer reads it. `pop` is only a key (the pop's address)
/// grouping regiments drawn from one pop; `supports` and `has` are the pop's own counts
/// (FUN_00965870 and FUN_00965930), the same the unit panel colours with.
#[derive(Clone, Copy, Debug, Default)]
pub struct Health {
    pub id: CfId,
    /// Unit type index; -1 when the catalogue cannot name it (never replaced).
    pub unit: i32,
    /// Percent of full strength; -1 when unreadable.
    pub strength: i32,
    pub pop: usize,
    pub supports: i32,
    pub has: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    Strength,
    Pop,
}

/// Indices into `regs` of the depleted regiments, in stack order, with why. A regiment
/// already being replaced (`busy`) is never chosen again. Low strength: below the
/// threshold. Pop too small: a pop that supports no regiment (red) gives up all its
/// regiments here; one that supports fewer than it has (yellow) gives up only the
/// difference, its last regiments in this stack, counting any already chosen.
pub fn depleted(regs: &[Health], choice: &Choice, busy: &[CfId]) -> Vec<(usize, Why)> {
    let named = |r: &Health| r.unit >= 0 && !busy.contains(&r.id);
    let mut why: Vec<Option<Why>> = vec![None; regs.len()];
    if choice.strength {
        for (i, r) in regs.iter().enumerate() {
            if named(r) && r.strength >= 0 && (r.strength as u32) < choice.below {
                why[i] = Some(Why::Strength);
            }
        }
    }
    if choice.pop {
        let mut done: Vec<usize> = Vec::new();
        for r in regs {
            if r.pop == 0 || done.contains(&r.pop) || (r.supports > 0 && r.supports >= r.has) {
                continue;
            }
            done.push(r.pop);
            let members: Vec<usize> = (0..regs.len()).filter(|&i| regs[i].pop == r.pop).collect();
            let excess = if r.supports <= 0 {
                members.len()
            } else {
                (r.has - r.supports) as usize
            };
            let mut taken = members
                .iter()
                .filter(|&&i| why[i].is_some() || busy.contains(&regs[i].id))
                .count();
            for &i in members.iter().rev() {
                if taken >= excess {
                    break;
                }
                if why[i].is_none() && named(&regs[i]) {
                    why[i] = Some(Why::Pop);
                    taken += 1;
                }
            }
        }
    }
    why.iter()
        .enumerate()
        .filter_map(|(i, w)| w.map(|w| (i, w)))
        .collect()
}

// ---------------------------------------------------------------- settings (local file)

const SETTINGS_HEADER: &str = "tgcx_replace_settings 2\r\n";

pub fn settings_write(c: &Choice) -> String {
    format!(
        "{SETTINGS_HEADER}strength={} pop={} below={} culture={}\r\n",
        u32::from(c.strength),
        u32::from(c.pop),
        c.below,
        c.culture
    )
}

/// Strict like every local file here: anything but the canonical text is refused.
pub fn settings_parse(text: &[u8]) -> Option<Choice> {
    let legacy = text.starts_with(b"tgcx_replace_settings 1\r\n");
    let header = if legacy {
        "tgcx_replace_settings 1\r\n"
    } else {
        SETTINGS_HEADER
    };
    let line = std::str::from_utf8(text.strip_prefix(header.as_bytes())?).ok()?;
    let rest = line.strip_prefix("strength=")?;
    let (strength, rest) = rest.split_once(" pop=")?;
    let (pop, below) = rest.split_once(" below=")?;
    let values = below.strip_suffix("\r\n")?;
    let (below, culture) = if legacy {
        (values, CULTURE_ALL)
    } else {
        let (below, culture) = values.split_once(" culture=")?;
        (below, culture.parse().ok()?)
    };
    let flag = |s: &str| match s {
        "0" => Some(false),
        "1" => Some(true),
        _ => None,
    };
    let c = Choice {
        strength: flag(strength)?,
        pop: flag(pop)?,
        below: below.parse().ok()?,
        culture,
    };
    let valid = (BELOW_MIN..=BELOW_MAX).contains(&c.below)
        && c.below.is_multiple_of(BELOW_STEP)
        && culture <= CULTURE_NONCORE;
    let canonical = if legacy {
        format!(
            "{header}strength={} pop={} below={}\r\n",
            u32::from(c.strength),
            u32::from(c.pop),
            c.below
        )
    } else {
        settings_write(&c)
    };
    (valid && canonical.as_bytes() == text).then_some(c)
}

// ---------------------------------------------------------------- saved replacements (TPL-7 style)

// A replacement being built, kept beside the Conform orders so a reload still detaches
// the old regiment once the new one merges. Same date window as those orders.
pub const REPLACE_MAX: usize = 512;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RpOrder {
    pub tag: u32,
    pub created: u32,
    pub done: u32,
    pub target: CfId,
    /// The joining stack (the regiment being built).
    pub stack: CfId,
    pub old: CfId,
    /// The replacement was seen in the target: only the old regiment's leaving is left.
    /// Kept open until it has left, so a save taken in between still detaches it.
    pub merged: bool,
    pub active: bool,
}

pub fn applies(r: &RpOrder, tag: u32, date: u32) -> bool {
    r.tag == tag && r.created <= date && (r.done == 0 || date < r.done)
}

/// What a load does with one saved record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Load {
    /// Another country, or a date outside the record's window: untouched.
    Skip,
    /// Its JOIN was resumed from the Conform orders: it carries the old regiment again.
    Resume,
    /// Merged, the old regiment still in the target: it leaves again.
    Retire,
    /// Nothing left to do, or nothing to resume it with: finished from the save's date.
    Close,
}

pub fn on_load(r: &RpOrder, applies: bool, joining: bool, old_in_target: bool) -> Load {
    if !applies {
        Load::Skip
    } else if r.merged {
        if old_in_target {
            Load::Retire
        } else {
            Load::Close
        }
    } else if joining {
        Load::Resume
    } else {
        Load::Close
    }
}

/// An active record with no JOIN left is finished, unless merged with the old regiment
/// still waiting to leave.
pub fn finished(r: &RpOrder, joining: bool, old_in_target: bool) -> bool {
    r.active && !joining && !(r.merged && old_in_target)
}

/// Full: the oldest finished record goes first, else the oldest open one. Returns the
/// new record's index and the index removed (later ones moved down one), if any.
pub fn add(k: &mut Vec<RpOrder>, r: RpOrder) -> (usize, Option<usize>) {
    let mut dropped = None;
    if k.len() >= REPLACE_MAX {
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
    k.push(RpOrder { active: false, ..r });
    (k.len() - 1, dropped)
}

fn line(r: &RpOrder) -> String {
    format!(
        "replace tag={:08X} created={} done={} target={}/{} stack={}/{} old={}/{} merged={}\r\n",
        r.tag,
        r.created,
        r.done,
        r.target.kind,
        r.target.id,
        r.stack.kind,
        r.stack.id,
        r.old.kind,
        r.old.id,
        u32::from(r.merged)
    )
}

/// Version 1 (0.11.7 to 0.11.12) had no `merged`: read as not merged, written as 2.
const HEADER: &str = "tgcx_replace_orders 2\r\n";
const HEADER_V1: &str = "tgcx_replace_orders 1\r\n";

/// The file's text, or None when it would not fit in `size` bytes.
pub fn write(k: &[RpOrder], size: usize) -> Option<String> {
    let mut out = String::from(HEADER);
    for r in k {
        out.push_str(&line(r));
    }
    (out.len() < size).then_some(out)
}

fn parse_line(text: &str) -> Option<RpOrder> {
    let mut r = RpOrder::default();
    let rest = text.strip_prefix("replace tag=")?;
    let (tag, rest) = rest.split_once(" created=")?;
    r.tag = u32::from_str_radix(tag, 16).ok()?;
    let (created, rest) = rest.split_once(" done=")?;
    r.created = created.parse().ok()?;
    let (done, rest) = rest.split_once(" target=")?;
    r.done = done.parse().ok()?;
    let (target, rest) = rest.split_once(" stack=")?;
    r.target = id_pair(target)?;
    let (stack, rest) = rest.split_once(" old=")?;
    r.stack = id_pair(stack)?;
    let old = match rest.split_once(" merged=") {
        Some((old, "0")) => old,
        Some((old, "1")) => {
            r.merged = true;
            old
        }
        Some(_) => return None,
        None => rest,
    };
    r.old = id_pair(old)?;
    Some(r)
}

/// Strict: every line must print back byte for byte, or the whole file is refused.
pub fn parse(text: &[u8]) -> Option<Vec<RpOrder>> {
    let text = text.split(|&c| c == 0).next().unwrap_or_default();
    let (legacy, mut at) = match text.strip_prefix(HEADER.as_bytes()) {
        Some(at) => (false, at),
        None => (true, text.strip_prefix(HEADER_V1.as_bytes())?),
    };
    let mut out = Vec::new();
    while !at.is_empty() {
        let end = at.windows(2).position(|w| w == b"\r\n")?;
        if end >= 190 || out.len() >= REPLACE_MAX {
            return None;
        }
        let r = parse_line(std::str::from_utf8(&at[..end]).ok()?)?;
        let canonical = match (legacy, r.merged) {
            (false, _) => line(&r),
            (true, false) => line(&r).replace(" merged=0", ""),
            (true, true) => return None,
        };
        if canonical.as_bytes() != &at[..end + 2]
            || r.target.none()
            || r.stack.none()
            || r.old.none()
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

    fn id(n: u32) -> CfId {
        CfId { kind: 7, id: n }
    }

    fn reg(n: u32, strength: i32, pop: usize, supports: i32, has: i32) -> Health {
        Health {
            id: id(n),
            unit: 0,
            strength,
            pop,
            supports,
            has,
        }
    }

    #[test]
    fn strength_below_the_threshold() {
        let regs = [
            reg(1, 49, 0, 0, 0),
            reg(2, 50, 0, 0, 0),
            reg(3, -1, 0, 0, 0),
        ];
        let only = Choice {
            pop: false,
            ..Choice::DEFAULT
        };
        assert_eq!(depleted(&regs, &only, &[]), vec![(0, Why::Strength)]);
        let off = Choice {
            strength: false,
            pop: false,
            below: 50,
            ..Choice::DEFAULT
        };
        assert!(depleted(&regs, &off, &[]).is_empty());
    }

    #[test]
    fn pop_red_gives_all_yellow_gives_the_difference() {
        // pop 100 is red (supports none): both its regiments. Pop 200 backs 3 and
        // supports 2: one, its last here. Pop 300 is fine.
        let regs = [
            reg(1, 100, 100, 0, 2),
            reg(2, 100, 200, 2, 3),
            reg(3, 100, 100, 0, 2),
            reg(4, 100, 200, 2, 3),
            reg(5, 100, 300, 3, 1),
        ];
        let only = Choice {
            strength: false,
            ..Choice::DEFAULT
        };
        assert_eq!(
            depleted(&regs, &only, &[]),
            vec![(0, Why::Pop), (2, Why::Pop), (3, Why::Pop)]
        );
    }

    #[test]
    fn a_weak_regiment_counts_toward_its_pop_and_busy_is_skipped() {
        // Pop 200 is one short: the weak first regiment already covers it.
        let regs = [reg(1, 20, 200, 1, 2), reg(2, 100, 200, 1, 2)];
        assert_eq!(
            depleted(&regs, &Choice::DEFAULT, &[]),
            vec![(0, Why::Strength)]
        );
        // Being replaced already: chosen by neither rule, and still counted for its pop.
        assert!(depleted(&regs, &Choice::DEFAULT, &[id(1)]).is_empty());
        // Unnamed units are never replaced.
        let unnamed = [Health {
            unit: -1,
            ..reg(9, 10, 0, 0, 0)
        }];
        assert!(depleted(&unnamed, &Choice::DEFAULT, &[]).is_empty());
    }

    #[test]
    fn slider_steps_stay_in_range() {
        let mut c = Choice::DEFAULT;
        assert_eq!(c.step(), 4);
        c.set_step(8);
        assert_eq!(c.below, 90);
        c.nudge(true);
        assert_eq!(c.below, 90);
        c.set_step(0);
        c.nudge(false);
        assert_eq!(c.below, 10);
        c.nudge(true);
        assert_eq!(c.below, 20);
    }

    #[test]
    fn settings_round_trip_and_refusals() {
        let c = Choice {
            strength: false,
            pop: true,
            below: 30,
            culture: CULTURE_NONCORE,
        };
        let text = settings_write(&c);
        assert_eq!(settings_parse(text.as_bytes()), Some(c));
        assert!(
            settings_parse(b"tgcx_replace_settings 1\r\nstrength=1 pop=1 below=55\r\n").is_none()
        );
        assert!(
            settings_parse(b"tgcx_replace_settings 1\r\nstrength=2 pop=1 below=50\r\n").is_none()
        );
        assert!(settings_parse(b"tgcx_replace_settings 1\r\nstrength=1 pop=1 below=50").is_none());
    }

    #[test]
    fn settings_import_legacy_and_reject_bad_cultures() {
        let old = b"tgcx_replace_settings 1\r\nstrength=0 pop=1 below=30\r\n";
        let c = settings_parse(old).unwrap();
        assert_eq!(
            c,
            Choice {
                strength: false,
                below: 30,
                ..Choice::DEFAULT
            }
        );
        assert_eq!(settings_parse(settings_write(&c).as_bytes()), Some(c));
        for mode in ["4", "01", "-1", "1 culture=2"] {
            let bad =
                format!("tgcx_replace_settings 2\r\nstrength=1 pop=1 below=50 culture={mode}\r\n");
            assert!(settings_parse(bad.as_bytes()).is_none());
        }
    }

    #[test]
    fn culture_choices_are_exclusive_and_clearable() {
        let mut c = Choice::DEFAULT;
        c.toggle_culture(1);
        assert_eq!(c.culture, 1);
        c.toggle_culture(CULTURE_NONCORE);
        assert_eq!(c.culture, CULTURE_NONCORE);
        c.toggle_culture(CULTURE_NONCORE);
        assert_eq!(c.culture, CULTURE_ALL);
        assert_eq!(c.below, 50);
        assert!(c.strength && c.pop);
    }

    #[test]
    fn orders_round_trip_window_and_refusals() {
        let r = RpOrder {
            tag: 0x415553,
            created: 100,
            done: 0,
            target: id(1),
            stack: id(2),
            old: id(3),
            merged: false,
            active: true,
        };
        let mut k = Vec::new();
        assert_eq!(add(&mut k, r), (0, None));
        let text = write(&k, 4096).unwrap();
        let back = parse(text.as_bytes()).unwrap();
        assert_eq!(back, vec![RpOrder { active: false, ..r }]);
        assert!(applies(&r, 0x415553, 100) && !applies(&r, 0x415553, 99));
        assert!(!applies(&RpOrder { done: 150, ..r }, 0x415553, 150));
        let bad = text.replace("old=7/3", "old=0/0");
        assert!(parse(bad.as_bytes()).is_none());
        assert!(parse(text.replace("\r\n", "\n").as_bytes()).is_none());
        assert!(write(&k, 10).is_none());
        let merged = vec![RpOrder {
            merged: true,
            ..back[0]
        }];
        let text = write(&merged, 4096).unwrap();
        assert!(text.ends_with("old=7/3 merged=1\r\n"));
        assert_eq!(parse(text.as_bytes()).unwrap(), merged);
        assert!(parse(text.replace("merged=1", "merged=2").as_bytes()).is_none());
        assert!(parse(text.replace(" merged=1", "").as_bytes()).is_none());
    }

    #[test]
    fn version1_orders_read_as_not_merged() {
        let v1 = "tgcx_replace_orders 1\r\nreplace tag=00415553 created=100 done=0 target=7/1 stack=7/2 old=7/3\r\n";
        let k = parse(v1.as_bytes()).unwrap();
        assert!(!k[0].merged && k[0].old == id(3));
        assert!(
            write(&k, 4096)
                .unwrap()
                .starts_with("tgcx_replace_orders 2\r\n")
        );
        assert!(parse(v1.replace("old=7/3", "old=7/3 merged=1").as_bytes()).is_none());
        assert!(parse(v1.replace("old=7/3", "old=7/3 merged=0").as_bytes()).is_none());
    }

    #[test]
    fn a_load_resumes_retires_or_closes() {
        let open = RpOrder {
            target: id(1),
            stack: id(2),
            old: id(3),
            ..RpOrder::default()
        };
        let merged = RpOrder {
            merged: true,
            ..open
        };
        assert_eq!(on_load(&open, false, true, true), Load::Skip);
        assert_eq!(on_load(&open, true, true, true), Load::Resume);
        // No JOIN carries it (its Conform order is gone): it would stay open forever.
        assert_eq!(on_load(&open, true, false, true), Load::Close);
        // Saved after the merge, before the old regiment left.
        assert_eq!(on_load(&merged, true, false, true), Load::Retire);
        assert_eq!(on_load(&merged, true, false, false), Load::Close);
    }

    #[test]
    fn a_merged_record_stays_open_until_the_old_regiment_leaves() {
        let open = RpOrder {
            active: true,
            ..RpOrder::default()
        };
        let merged = RpOrder {
            merged: true,
            ..open
        };
        assert!(!finished(&open, true, true));
        assert!(finished(&open, false, true));
        assert!(!finished(&merged, false, true));
        assert!(finished(&merged, false, false));
        assert!(!finished(
            &RpOrder {
                active: false,
                ..merged
            },
            false,
            false
        ));
    }

    #[test]
    fn a_full_table_drops_the_oldest_finished_record() {
        let mut k: Vec<RpOrder> = (0..REPLACE_MAX as u32)
            .map(|i| RpOrder {
                created: 1000 + i,
                done: if i == 7 { 2000 } else { 0 },
                target: id(1),
                stack: id(i + 10),
                old: id(i + 5000),
                ..RpOrder::default()
            })
            .collect();
        let next = RpOrder {
            created: 3000,
            ..k[0]
        };
        let (index, dropped) = add(&mut k, next);
        assert_eq!((index, dropped), (REPLACE_MAX - 1, Some(7)));
    }
}
