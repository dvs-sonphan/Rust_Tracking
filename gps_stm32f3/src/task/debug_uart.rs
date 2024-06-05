use core::borrow::{Borrow, BorrowMut};
// #[warn(unused_imports)]
use core::fmt::Write;
use defmt::*;
use defmt_rtt as _;
// use embassy_executor::task;

use embassy_stm32::peripherals::USART1;
use embassy_stm32::usart::{Config, Uart, UartTx};
use embassy_stm32::{bind_interrupts, interrupt, peripherals, usart, Peripherals};
use heapless::String;
// use {defmt_rtt as _, panic_probe as _};
// use panic_halt as _;
// use embassy_executor::task;
// use embassy_stm32::dma::NoDma;
// use embassy_stm32::usart::{self, Uart};
// use embassy_stm32::{interrupt, Peripherals};
// use embassy_stm32::usart::{Config, Uart, BasicInstance};
use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5};
// use embassy_stm32::interrupt::Irqs;

// pub type NetworkUart = embassy_stm32::peripherals::USART1;
// pub type NetworkUartInterrupt = USART1;
// pub type NetworkUartRxPin = embassy_stm32::peripherals::PA10;
// pub type NetworkUartTxPin = embassy_stm32::peripherals::PA9;
// pub type NetworkUartRxDma = embassy_stm32::peripherals::DMA1_CH5;
// pub type NetworkUartTxDma = embassy_stm32::peripherals::DMA1_CH4;

// bind_interrupts!(struct Irqs {
//     USART1 => usart::InterruptHandler<peripherals::USART1>;
// });

// pub struct NetworkPeripherals {
//     pub uart: NetworkUart,
//     pub uart_interrupt: NetworkUartInterrupt,
//     pub uart_rx_pin: NetworkUartRxPin,
//     pub uart_tx_pin: NetworkUartTxPin,
//     pub uart_rx_dma: NetworkUartRxDma,
//     pub uart_tx_dma: NetworkUartTxDma,
//     // uart_interrupt: Irqs,
// }

// pub fn init(p: Peripherals) -> NetworkPeripherals {
//     NetworkPeripherals {
//         uart: p.USART2,
//         uart_interrupt: interrupt::take!(USART2),
//         uart_rx_pin: p.PD6,
//         uart_tx_pin: p.PD5,
//         uart_rx_dma: p.DMA1_CH5,
//         uart_tx_dma: p.DMA1_CH6,
//     }
// }

// #[allow(dead_code)]
// // pub fn init_peripheral(p: Uart<'static, USART1, DMA1_CH4, DMA1_CH5>) {
// pub fn init_peripheral(p: Peripherals) -> Uart<'static, USART1, DMA1_CH4, DMA1_CH5> {
//     // let p = embassy_stm32::init(Default::default());

//     let mut config_debug = Config::default();
//     config_debug.baudrate = 115200;

//     // let usart1 = &p.USART1;
//     let uart = Uart::new(
//         // *usart1,
//         p.USART1,
//         p.PA10,
//         p.PA9,
//         Irqs,
//         p.DMA1_CH4,
//         p.DMA1_CH5,
//         config_debug,
//     )
//     .unwrap();

//     uart
// }

// #[embassy_executor::task]
pub async fn show_data_debug(uart: &mut Uart<'static, USART1, DMA1_CH4, DMA1_CH5>, message: &str) {
    // let p = embassy_stm32::init(Default::default());
    info!("Write Command");

    // let mut config_debug = Config::default();
    // config_debug.baudrate = 115200;

    // let mut usart_debug_tx = UartTx::new(p.USART1, p.PA9, NoDma, config_debug).unwrap();

    // let mut usart_debug_tx = Uart::new(
    //     p.USART1,
    //     p.PA10,
    //     p.PA9,
    //     Irqs,
    //     p.DMA1_CH4,
    //     p.DMA1_CH5,
    //     config_debug,
    // )
    // .unwrap();

    // let mut s: String<128> = String::new();
    // core::write!(&mut s, "{}\r\n", message).unwrap();

    // let _ = usart_debug_tx.write(s.as_bytes()).await;
    // usart_debug_tx.write(s.as_bytes()).await.unwrap();

    uart.write(message.as_bytes()).await.unwrap();
    // uart.blocking_write(message.as_bytes()).unwrap();
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

// #[embassy_executor::task]
// pub async fn read_command() {
//     let p = embassy_stm32::init(Default::default());
//     info!("Read Command");

//     // bind_interrupts!(struct Irqs {
//     //     USART1 => usart::InterruptHandler<peripherals::USART1>;
//     // });

//     let config = Config::default();
//     let mut usart = Uart::new(
//         p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH4, p.DMA1_CH5, config,
//     )
//     .unwrap();

//     let mut msg: [u8; 8] = [0; 8];

//     loop {
//         usart.read(&mut msg).await.unwrap();
//         usart.write(&msg).await.unwrap();
//     }
// }

// #[embassy_executor::task]
// pub async fn show_data_hello() {
//     // loop {
//     //     info!("Hello");
//     // }
//     info!("Hello");
// }
