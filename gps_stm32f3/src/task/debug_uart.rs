// #[warn(unused_imports)]
use core::fmt::Write;

use defmt::*;
use defmt_rtt as _;

use embassy_stm32::usart::{Config, Uart, UartTx};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use heapless::String;
// use {defmt_rtt as _, panic_probe as _};
// use panic_halt as _;
// use embassy_executor::task;
// use embassy_stm32::dma::NoDma;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

// #[embassy_executor::task]
pub async fn show_data_debug(message: &str) {
    let p = embassy_stm32::init(Default::default());
    info!("Write Command");

    let mut config_debug = Config::default();
    config_debug.baudrate = 115200;

    // let mut usart_debug_tx = UartTx::new(p.USART1, p.PA9, NoDma, config_debug).unwrap();

    let mut usart_debug_tx = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        Irqs,
        p.DMA1_CH4,
        p.DMA1_CH5,
        config_debug,
    )
    .unwrap();

    // let mut s: String<128> = String::new();
    // core::write!(&mut s, "{}\r\n", message).unwrap();

    // let _ = usart_debug_tx.write(s.as_bytes()).await;
    // usart_debug_tx.write(s.as_bytes()).await.unwrap();

    // usart_debug_tx.write(message.as_bytes()).await.unwrap();
    usart_debug_tx.blocking_write(message.as_bytes()).unwrap();
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
//         Err(_e) => {
//             // info!("Failed to convert byte array to &str: {}", e);
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
