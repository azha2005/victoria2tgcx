//! Bounded, sampled plugin CPU-wall timing. No engine state or permission cache.
//! Hot callbacks sample one in 1024 entries; daily/visibility work samples all.
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

pub struct Timing {
    calls: AtomicU32,
    samples: AtomicU32,
    micros: AtomicU32,
    max_micros: AtomicU32,
    mask: u32,
}
impl Timing {
    const fn new(mask: u32) -> Self {
        Self {
            calls: AtomicU32::new(0),
            samples: AtomicU32::new(0),
            micros: AtomicU32::new(0),
            max_micros: AtomicU32::new(0),
            mask,
        }
    }
    pub fn begin(&self) -> Span<'_> {
        let count = self.calls.fetch_add(1, Ordering::Relaxed);
        Span {
            timing: self,
            start: (count & self.mask == 0).then(Instant::now),
        }
    }
    fn record(&self, micros: u32) {
        self.samples.fetch_add(1, Ordering::Relaxed);
        // Saturation keeps a long session's diagnostics from wrapping silently.
        let _ = self
            .micros
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                Some(n.saturating_add(micros))
            });
        self.max_micros.fetch_max(micros, Ordering::Relaxed);
    }
    fn report(&self, name: &str) -> String {
        format!(
            "{name}:calls={},samples={},sample_us={},max_us={}",
            self.calls.load(Ordering::Relaxed),
            self.samples.load(Ordering::Relaxed),
            self.micros.load(Ordering::Relaxed),
            self.max_micros.load(Ordering::Relaxed)
        )
    }
}
pub struct Span<'a> {
    timing: &'a Timing,
    start: Option<Instant>,
}
impl Drop for Span<'_> {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            self.timing
                .record(start.elapsed().as_micros().min(u32::MAX as u128) as u32);
        }
    }
}
pub static PAIR: Timing = Timing::new(1023);
pub static SIDE: Timing = Timing::new(1023);
pub static ACCESS: Timing = Timing::new(1023);
pub static EXILE: Timing = Timing::new(1023);
pub static SIEGE: Timing = Timing::new(1023);
pub static JOIN: Timing = Timing::new(1023);
pub static VISION: Timing = Timing::new(0);
pub static OBSERVER: Timing = Timing::new(0);
pub static DAY: Timing = Timing::new(0);
pub static GUI: Timing = Timing::new(63);
pub fn report() -> String {
    [
        ("pair", &PAIR),
        ("side", &SIDE),
        ("access", &ACCESS),
        ("exile", &EXILE),
        ("siege", &SIEGE),
        ("join", &JOIN),
        ("vision", &VISION),
        ("observer", &OBSERVER),
        ("day", &DAY),
        ("gui", &GUI),
    ]
    .iter()
    .map(|(name, timing)| timing.report(name))
    .collect::<Vec<_>>()
    .join(" ")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sampling_is_bounded_and_accumulation_saturates() {
        let timing = Timing::new(1023);
        let mut sampled = 0;
        for _ in 0..4096 {
            let span = timing.begin();
            sampled += usize::from(span.start.is_some());
        }
        assert_eq!(sampled, 4);
        assert_eq!(timing.samples.load(Ordering::Relaxed), 4);
        timing.record(u32::MAX);
        timing.record(10);
        assert_eq!(timing.micros.load(Ordering::Relaxed), u32::MAX);
        assert_eq!(timing.max_micros.load(Ordering::Relaxed), u32::MAX);
    }
}
