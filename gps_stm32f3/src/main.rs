// main.rs
#![no_std]
#![no_main]

mod debug_uart;

use embassy_executor::Spawner;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello");
    // debug_uart::debug_uart("Hello from main!").unwrap();
    debug_uart::debug_uart("Test UART").await;
}
