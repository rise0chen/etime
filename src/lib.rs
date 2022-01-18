#![no_std]

use core::ops::Range;
use core::sync::atomic::{AtomicU64, Ordering};
use core::time::Duration;

pub struct Etime {
    start: AtomicU64,
}
impl Etime {
    pub const fn new() -> Self {
        Self {
            start: AtomicU64::new(0),
        }
    }
    // ns
    #[inline]
    pub fn now(&self) -> u64 {
        clock_source::now().as_nanos() as u64
    }
    pub fn tic(&self) {
        let now = self.now();
        self.start.store(now, Ordering::Relaxed);
    }
    pub fn toc(&self) -> Duration {
        let now = self.now();
        let start = self.start.load(Ordering::Relaxed);
        Duration::from_nanos(now - start)
    }
}

pub fn expect_time<F: FnOnce(Duration)>(val: Duration, expect: Range<Duration>, on_failed: F) {
    if !expect.contains(&val) {
        on_failed(val);
    }
}
