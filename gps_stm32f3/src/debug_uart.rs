#[warn(unused_imports)]
use core::fmt::Write;

use defmt::*;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};
// use panic_halt as _;
// use embassy_executor::task;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::task]
pub async fn write_command(text: &'static str) {
    let p = embassy_stm32::init(Default::default());
    info!("Write Command");

    // bind_interrupts!(struct Irqs {
    //     USART1 => usart::InterruptHandler<peripherals::USART1>;
    // });

    let config = Config::default();
    let mut usart = Uart::new(
        p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH4, p.DMA1_CH5, config,
    )
    .unwrap();

    let mut s: String<128> = String::new();
    core::write!(&mut s, "{}\r\n", text).unwrap();

    let _ = usart.write(s.as_bytes()).await;

    // loop {
    //     let _ = usart.write(b"OK\r\n").await;
    // }
}

// #[embassy_executor::task]
// pub async fn write_command(text: &'static [u8; 8]) {
//     let p = embassy_stm32::init(Default::default());
//     info!("Write Command");

//     bind_interrupts!(struct Irqs {
//         USART1 => usart::InterruptHandler<peripherals::USART1>;
//     });

//     let config = Config::default();
//     let mut usart = Uart::new(
//         p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH4, p.DMA1_CH5, config,
//     )
//     .unwrap();

//     // Attempt to convert the byte array to a &str
//     let text_str = match core::str::from_utf8(text) {
//         Ok(s) => s,
//         Err(e) => {
//             info!("Failed to convert byte array to &str: {}", e);
//             return;
//         }
//     };

//     let mut s: String<128> = String::new();
//     core::write!(&mut s, "{}\r\n", text_str).unwrap();

//     usart.write(s.as_bytes()).await.unwrap();
//     info!("wrote DMA");
// }

#[embassy_executor::task]
pub async fn read_command() {
    let p = embassy_stm32::init(Default::default());
    info!("Read Command");

    // bind_interrupts!(struct Irqs {
    //     USART1 => usart::InterruptHandler<peripherals::USART1>;
    // });

    let config = Config::default();
    let mut usart = Uart::new(
        p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH4, p.DMA1_CH5, config,
    )
    .unwrap();

    let mut msg: [u8; 8] = [0; 8];

    loop {
        usart.read(&mut msg).await.unwrap();
        usart.write(&msg).await.unwrap();
    }
}
