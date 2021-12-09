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

#[derive(Default)]
pub struct Config {
    pub expect: Range<Duration>,
    pub success: Option<fn(&Duration)>,
    pub failed: Option<fn(&Duration)>,
}
pub struct Etime {
    cfg: Config,
    start: AtomicU64,
}
impl Etime {
    pub fn new() -> Self {
        Self {
            cfg: Config::default(),
            start: AtomicU64::new(0),
        }
    }
    pub fn set_config(&mut self, cfg: Config) {
        self.cfg = cfg
    }

    pub fn tic(&self) {
        let now = unsafe { CLOCK() };
        self.start.store(now, Ordering::Relaxed);
    }
    pub fn toc(&self) -> Duration {
        let now = unsafe { CLOCK() };
        let start = self
            .start
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| Some(0))
            .unwrap();
        let elapsed = Duration::from_nanos(now - start);
        if self.cfg.expect.contains(&elapsed) {
            if let Some(f) = self.cfg.success {
                f(&elapsed);
            }
        } else {
            if let Some(f) = self.cfg.failed {
                f(&elapsed);
            }
        }
        elapsed
    }
}
