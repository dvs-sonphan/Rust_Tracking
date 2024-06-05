// #[warn(unused_must_use)]

// use core::fmt::Write;

use crate::task::debug_uart;
// mod task::debug_uart;

use defmt::*;
use defmt_rtt as _;

// use embassy_stm32::dma::NoDma;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart, Peripherals};

// use super::debug_uart;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;

static SHARED: Mutex<ThreadModeRawMutex, u32> = Mutex::new(0);

// use heapless::{String,Vec};
// use {defmt_rtt as _, panic_probe as _};

// bind_interrupts!(struct Irqs {
//     USART2 => usart::InterruptHandler<peripherals::USART2>;
// });

#[embassy_executor::task]
pub async fn read_data_gps(power_pin: peripherals::PA4, mut gps_uart: Uart<'static, peripherals::USART2, peripherals::DMA1_CH7, peripherals::DMA1_CH6>) {
// pub async fn read_data_gps(power: peripherals::PA4, gps_uart: Uart<'static, USART2, DMA1_CH7, DMA1_CH6>) {
    // pub async fn read_data_gps() {
    // let p = embassy_stm32::init(Default::default());
    info!("Task GPS");

    //Turn on power for module GPS
    // let _gps_pwr = Output::new(p.PA4, Level::High, Speed::VeryHigh);
    let _gps_pwr = Output::new(power_pin, Level::High, Speed::VeryHigh);

    // let mut config = Config::default();
    // config.baudrate = 9600;

    // let mut usart = Uart::new(
    //     // p.USART2, p.PA3, p.PA2, Irqs
    //     p.USART2, p.PA3, p.PA2, Irqs, p.DMA1_CH7, p.DMA1_CH6, config,
    // )
    // .unwrap();

    //Test debug UART
    // debug_uart::show_data_debug("UART Debug").await;
    // let _ = debug_uart::read_command();

    let mut msg: [u8; 128] = [0; 128];

    // let value = debug_uart::init_peripheral(p);
    loop {
        let mut shared = SHARED.lock().await;
        *shared = shared.wrapping_add(1);

        // usart.read(&mut msg).await.unwrap();
        gps_uart.read(&mut msg).await.unwrap();
        // usart.write(&msg).await.unwrap();

        // let _ = debug_uart::write_command("UART GPS");
        // let s = str::from_utf8(&msg);
        if let Ok(message) = core::str::from_utf8(&msg) {
            println!("result: {}", message);
            // self::debug_uart::show_data_debug(message).await;

            // let mut debug_peripherals = value;
            // let _ = debug_peripherals.write(b"Test UART\r\n").await;
        }

        // let _ = debug_uart::read_command();
    }
}

//Test
// #[embassy_executor::task]
// pub async fn gps_main_task() {
//     let p = embassy_stm32::init(Default::default());
//     info!("Task GPS");

//     bind_interrupts!(struct IrqsUART1 {
//         USART1 => usart::InterruptHandler<peripherals::USART1>;
//     });

//     bind_interrupts!(struct IrqsUART2 {
//         USART2 => usart::InterruptHandler<peripherals::USART2>;
//     });

//     //Turn on power for module GPS
//     let _gps_pwr = Output::new(p.PA4, Level::High, Speed::VeryHigh);

//     //GPS config
//     let mut config_gps = Config::default();
//     config_gps.baudrate = 9600;

//     let mut usart_gps = Uart::new(
//         p.USART2, p.PA3, p.PA2, IrqsUART2, p.DMA1_CH7, p.DMA1_CH6, config_gps,
//     )
//     .unwrap();

//     //Debug UART config
//     let mut config_debug = Config::default();
//     config_debug.baudrate = 115200;

//     let mut usart_debug = Uart::new(
//         p.USART1,
//         p.PA10,
//         p.PA9,
//         IrqsUART1,
//         p.DMA1_CH4,
//         p.DMA1_CH5,
//         config_debug,
//     )
//     .unwrap();

//     let mut msg: [u8; 128] = [0; 128];
//     // let mut msg: Vec<u8, 128> = Default::default();

//     usart_debug.write(b"GPS Task\r\n").await.unwrap();

//     loop {
//         usart_gps.read(&mut msg).await.unwrap();
//         // usart_debug.write(&msg).await.unwrap();

//         // Convert msg to a string, ignoring invalid UTF-8 sequences
//         if let Ok(message) = core::str::from_utf8(&msg) {
//             // Check the length of the message and return if it is 1024 characters long
//             if message.len() == 128 {
//                 usart_debug.write(message.as_bytes()).await.unwrap();
//                 // Convert the message to heapless::String<1024>
//                 // let mut heapless_string: String<128> = String::new();
//                 // if heapless_string.push_str(message).is_ok() {
//                 // return Some(heapless_string);
//                 // let _ = write_command(heapless_string);
//                 // usart_debug.write(message.as_bytes()).await.unwrap();
//                 // }
//                 // let _ = write_command(message);
//             }
//         }
//     }
// }
