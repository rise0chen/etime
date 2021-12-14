use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static mut CLOCK: fn() -> u64 = || {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_nanos() as u64
};
pub unsafe fn set_clock_source(f: fn() -> u64) {
    CLOCK = f;
}

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
    pub fn now(&self) -> u64 {
        unsafe { CLOCK() }
    }
    pub fn tic(&self) {
        let now = unsafe { CLOCK() };
        self.start.store(now, Ordering::Relaxed);
    }
    pub fn toc(&self) -> Duration {
        let now = unsafe { CLOCK() };
        let start = self.start.load(Ordering::Relaxed);
        Duration::from_nanos(now - start)
    }
}

pub fn expect_time(val: Duration, expect: Range<Duration>, on_failed: fn(&Duration)) {
    if !expect.contains(&val) {
        on_failed(&val);
    }
}
