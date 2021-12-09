use core::time::Duration;
use etime::{Config, Etime};
use std::thread;

fn main() {
    let mut etime = Etime::new();
    etime.set_config(Config {
        expect: Duration::from_secs(2)..Duration::from_secs(5),
        success: Some(|elapsed| println!("success: {:?}", elapsed)),
        failed: Some(|elapsed| println!("failed: {:?}", elapsed)),
    });
    etime.tic();
    thread::sleep(Duration::from_secs(1));
    etime.toc();
    etime.tic();
    thread::sleep(Duration::from_secs(3));
    etime.toc();
    etime.tic();
    thread::sleep(Duration::from_secs(6));
    etime.toc();
}
