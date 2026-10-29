//! Bounded UI-only native route search. Cached answers never authorize Send.
use crate::model::Identity;
use std::collections::{BTreeMap, VecDeque};

#[derive(Default)]
pub struct Chooser {
    pending: VecDeque<(Identity, VecDeque<u32>)>,
    pub targets: BTreeMap<Identity, u32>,
}
impl Chooser {
    pub fn start(armies: impl IntoIterator<Item = (Identity, Vec<u32>)>) -> Self {
        Self {
            pending: armies
                .into_iter()
                .map(|(id, targets)| (id, targets.into()))
                .collect(),
            targets: BTreeMap::new(),
        }
    }
    pub fn advance(&mut self, budget: usize, mut query: impl FnMut(Identity, u32) -> Option<bool>) {
        let mut used = 0;
        while used < budget {
            let Some((army, mut destinations)) = self.pending.pop_front() else {
                break;
            };
            let Some(target) = destinations.pop_front() else {
                continue;
            };
            used += 1;
            if query(army, target) == Some(true) {
                self.targets.insert(army, target);
            } else if !destinations.is_empty() {
                // Round-robin so a large unreachable army does not starve a
                // nearby valid one while it exhausts hundreds of candidates.
                self.pending.push_back((army, destinations));
            }
        }
    }
    pub fn pending(&self) -> usize {
        self.pending.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scans_all_destinations_in_bounded_batches_and_never_accepts_unknown() {
        let a = Identity { kind: 40, id: 1 };
        let b = Identity { kind: 40, id: 2 };
        let mut chooser = Chooser::start([(a, (1..=300).collect()), (b, vec![10, 20])]);
        let mut calls = 0;
        for _ in 0..40 {
            let before = calls;
            chooser.advance(8, |army, target| {
                calls += 1;
                if army == a { Some(target == 299) } else { None }
            });
            assert!(calls - before <= 8);
        }
        assert_eq!(chooser.targets.get(&a), Some(&299));
        assert!(!chooser.targets.contains_key(&b));
        assert_eq!(chooser.pending(), 0);
        chooser.advance(8, |_, _| panic!("completed cache queried"));
    }
    #[test]
    fn unreachable_first_army_cannot_starve_a_reachable_one() {
        let a = Identity { kind: 40, id: 1 };
        let b = Identity { kind: 40, id: 2 };
        let mut chooser = Chooser::start([(a, (1..=65535).collect()), (b, vec![100])]);
        chooser.advance(2, |army, _| Some(army == b));
        assert_eq!(chooser.targets.get(&b), Some(&100));
        assert_eq!(chooser.pending(), 1);
    }
}
