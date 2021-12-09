# Etime

## Usage

### Base

```rust
use core::time::Duration;
use etime::Etime;
use std::thread;

let etime = Etime::new();
etime.tic();
thread::sleep(Duration::from_secs(1));
let elapsed = etime.toc();
println!("{:?}", elapsed);
```

### Set Clock Source

### Functions

```rust
let mut etime = Etime::new();
etime.set_config(Config {
    expect: Duration::from_secs(2)..Duration::from_secs(5),
    success: Some(|elapsed| println!("success: {:?}", elapsed)),
    failed: Some(|elapsed| println!("failed: {:?}", elapsed)),
});
etime.tic();
thread::sleep(Duration::from_secs(1));
etime.toc();
```