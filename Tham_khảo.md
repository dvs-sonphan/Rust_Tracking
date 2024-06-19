## Async Rust
- https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown
- https://dev.to/theembeddedrustacean/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives-59hk
- https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html
- https://www.linkedin.com/pulse/tokio-runtime-writing-reliable-asynchronous-withrust-soares-m-sc-/

### [embassy_executor](https://docs.embassy.dev/embassy-executor/git/cortex-m/index.html)
> An async/await executor designed for embedded usage.

### WDT embassy
- https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/wdt.rs
- https://github.com/embassy-rs/embassy/blob/main/examples/stm32h7/src/bin/wdg.rs

----------------------------------------------------------------------------------
## Tham khảo một số code dùng thư viện Embassy

### https://github.com/titanclass/embassy-start/blob/master/server/embedded-app/src/network.rs
- Đây là đoạn code mình thấy có **đọc và viết** qua **uart**.
- Sử dụng thêm các thư viện:
```
postcard = "1.0"
heapless = { version = "0.7.7", features = ["defmt-impl", "serde"] }
serde = { version = "1.0.126", default-features = false, features = ["derive"] }
```

### https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/uart_r503.rs

----------------------------------------------------------------------------------
### [nmea_parser](https://docs.rs/nmea-parser/latest/nmea_parser/)
- https://crates.io/crates/nmea-parser
- https://github.com/zaari/nmea-parser/tree/master

## Một số project về GPS
- https://github.com/tstellanova/ublox-core
- https://github.com/BlackbirdHQ/ublox-cellular-rs
- https://github.com/ublox-rs/ublox
- https://github.com/MechanicalPython/adafruit_gps

---------------------------------------------------------------------------------
### macro
- https://users.rust-lang.org/t/write-macro-doesnt-work-in-no-std/40343
- https://os.phil-opp.com/heap-allocation/
- https://blog.timhutt.co.uk/std-embedded-rust/index.html

--------------------------------------------------------------------------------
### Channel Community
- https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/blinky_two_tasks.rs
- https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/blinky_two_channels.rs
- https://rust-classes.com/chapter_embedded_stmf401_soil
- https://dhghomon.github.io/easy_rust/Chapter_50.html

--------------------------------------------------------------------------------
### Vector
- https://fasterthanli.me/articles/a-half-hour-to-learn-rust#example-vec-t
- https://dhghomon.github.io/easy_rust/Chapter_21.html

--------------------------------------------------------------------------------
### Method (Phương thức)
- https://tourofrust.com/24_vi.html
- https://fasterthanli.me/articles/a-half-hour-to-learn-rust#methods













