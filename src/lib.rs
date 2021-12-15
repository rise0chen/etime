use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const CLOCK: fn() -> u64 = || {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_nanos() as u64
};

pub struct Etime {
    start: AtomicU64,
    clock: fn() -> u64,
}
impl Etime {
    pub const fn new() -> Self {
        Self {
            start: AtomicU64::new(0),
            clock: CLOCK,
        }
    }
    pub fn set_clock(&mut self, clock: fn() -> u64) {
        self.clock = clock;
    }
    // ns
    #[inline]
    pub fn now(&self) -> u64 {
        (self.clock)()
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

pub fn expect_time(val: Duration, expect: Range<Duration>, on_failed: fn(&Duration)) {
    if !expect.contains(&val) {
        on_failed(&val);
    }
}
