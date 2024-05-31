## Async Rust
```
https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown

https://dev.to/theembeddedrustacean/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives-59hk

https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

https://www.linkedin.com/pulse/tokio-runtime-writing-reliable-asynchronous-withrust-soares-m-sc-/
```

### [embassy_executor](https://docs.embassy.dev/embassy-executor/git/cortex-m/index.html)
> An async/await executor designed for embedded usage.

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











