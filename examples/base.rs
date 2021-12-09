use core::time::Duration;
use etime::Etime;
use std::thread;

fn main() {
    let etime = Etime::new();
    etime.tic();
    thread::sleep(Duration::from_secs(1));
    let elapsed = etime.toc();
    println!("{:?}", elapsed);
}
