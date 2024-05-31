// use core::fmt::Write;

use core::str;


use crate::debug_uart;

use defmt::*;
// use embassy_stm32::dma::NoDma;
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
// use heapless::{String,Vec};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::task]
pub async fn read_data_gps(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Task GPS");

    let config = Config::default();
    let mut usart = Uart::new(
        // p.USART2, p.PA3, p.PA2, Irqs
        p.USART2, p.PA3, p.PA2, Irqs, p.DMA1_CH7, p.DMA1_CH6, config,
    )
    .unwrap();

    let mut msg: [u8; 8] = [0; 8];

    loop {
        usart.read(&mut msg).await.unwrap();
        usart.write(&msg).await.unwrap();

        // let _ = debug_uart::write_command("UART GPS");
        let s = str::from_utf8(&msg);

        println!("result: {}", s);
        spawner.spawn(debug_uart::write_command(s));
    }
}
