// main.rs
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// mod debug_uart;
// mod task_gps;
// use task_gps::read_data_gps;
// use panic_halt as _;
mod task;

// use core::fmt::Write;

use core::fmt::Write;

use defmt::*;
use defmt_rtt as _;

use embassy_executor::Spawner;
// use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5, USART1};

use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart, Peripherals};
use heapless::String;
// use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use panic_probe as _;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;

static SHARED: Mutex<ThreadModeRawMutex, u32> = Mutex::new(0);

bind_interrupts!(struct IrqsUART1 {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

bind_interrupts!(struct IrqsUART2 {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

// use embassy_stm32::usart::{Config, Uart};
// use embassy_stm32::{bind_interrupts, peripherals, usart};
// use heapless::String;
// use panic_halt as _;
// use embassy_executor::task;

// bind_interrupts!(struct IrqsUART1 {
//     USART1 => usart::InterruptHandler<peripherals::USART1>;
// });

// bind_interrupts!(struct IrqsUART2 {
//     USART2 => usart::InterruptHandler<peripherals::USART2>;
// });

// #[embassy_executor::task]
// async fn read_data_gps() {
//     let p = embassy_stm32::init(Default::default());
//     // info!("Task GPS");

//     //Turn on power for module GPS
//     let _gps_pwr = Output::new(p.PA4, Level::High, Speed::VeryHigh);
//     // let _gps_pwr = Output::new(p.PA4, Level::Low, Speed::VeryHigh);

//     let mut config = Config::default();
//     config.baudrate = 9600;

//     let mut usart = Uart::new(
//         // p.USART2, p.PA3, p.PA2, IrqsUART2, p.DMA1_CH7, p.DMA1_CH6, config,
//         p.USART2, p.PA3, p.PA2, IrqsUART2, p.DMA1_CH7, p.DMA1_CH6, config,
//     )
//     .unwrap();

//     let mut msg: [u8; 128] = [0; 128];
//     // let mut msg: Vec<u8, 128> = Default::default();

//     loop {
//         usart.read(&mut msg).await.unwrap();
//         // usart.write(&msg).await.unwrap();

//         // Convert msg to a string, ignoring invalid UTF-8 sequences
//         if let Ok(message) = core::str::from_utf8(&msg) {
//             // Check the length of the message and return if it is 1024 characters long
//             if message.len() == 128 {
//                 // Convert the message to heapless::String<1024>
//                 let mut heapless_string: String<128> = String::new();
//                 if heapless_string.push_str(message).is_ok() {
//                     // return Some(heapless_string);
//                     // let _ = write_command(heapless_string);
//                 }
//                 // let _ = write_command(message);
//             }
//         }

//         // // Convert msg to a string, ignoring invalid UTF-8 sequences
//         // if let Ok(message) = String::from_utf8(msg) {
//         //     // Check the length of the message and return if it is 1024 characters long
//         //     if message.len() == 128 {
//         //         // return message.to_string();
//         //         write_command(message);
//         //     }
//         // }
//     }

// }

// #[embassy_executor::task]
// // async fn write_command(text: String<128>) {
// // async fn write_command(text: &str) {
// async fn write_command(text: &'static str) {
//     let p = embassy_stm32::init(Default::default());
//     // info!("Write Data");
//     // rprint!("Write Data");

//     let mut config = Config::default();
//     config.baudrate = 115200;

//     let mut usart = Uart::new(
//         p.USART1, p.PA10, p.PA9, IrqsUART1, p.DMA1_CH4, p.DMA1_CH5, config,
//     )
//     .unwrap();

//     // let mut msg: [u8; 8] = [0; 8];

//     // loop {
//     //     usart.read(&mut msg).await.unwrap();
//     //     usart.write(&msg).await.unwrap();
//     // }
//     // defmt::println!("Test UART");

//     // let mut s: String<128> = String::new();
//     // core::write!(&mut s, "{}\r\n", text).unwrap();

//     // defmt::info!("Test UART" );

//     usart.write(text.as_bytes()).await.unwrap();
//     usart.write(b"Starting UART\r\n").await.unwrap();
//     // let _ = usart.write(s.as_bytes()).await;
// }

// #[embassy_executor::task]
// async fn gps_main_task() {
//     let p = embassy_stm32::init(Default::default());
//     // info!("Task GPS");

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
//         p.USART1, p.PA10, p.PA9, IrqsUART1, p.DMA1_CH4, p.DMA1_CH5, config_debug,
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
//                     // return Some(heapless_string);
//                     // let _ = write_command(heapless_string);
//                     // usart_debug.write(message.as_bytes()).await.unwrap();
//                 // }
//                 // let _ = write_command(message);
//             }
//         }
//     }

// }
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

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello");
    let p = embassy_stm32::init(Default::default());

    //************ Debug UART config ************************
    let mut config_debug = Config::default();
    config_debug.baudrate = 115200;

    let mut usart_debug = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        IrqsUART1,
        p.DMA1_CH4,
        p.DMA1_CH5,
        config_debug,
    )
    .unwrap();

    //***************** GPS config *************************
    let mut config_gps = Config::default();
    config_gps.baudrate = 9600;

    let usart_gps = Uart::new(
        p.USART2, p.PA3, p.PA2, IrqsUART2, p.DMA1_CH7, p.DMA1_CH6, config_gps,
    )
    .unwrap();

    //******************* Task***************************
    // let debug_uart = init_peripheral(p);
    task::debug_uart::show_data_debug(&mut usart_debug, "Test GPS").await;
    task::debug_uart::show_data_debug(&mut usart_debug, "GPS Data: ").await;

    //GPS Task
    spawner.spawn(task::task_gps::read_data_gps(p.PA4, usart_gps)).unwrap();

    // STM32
    // let mut debug_peripherals = task::debug_uart::init_peripheral(&p);
    // let _ = debug_peripherals.write(b"Test UART\r\n").await;

    // let mut s: String<128> = String::new();
    // core::write!(&mut s, "{}\r\n", "GPS Task.....................................................................................").unwrap();
    // let _ = task::debug_uart::write_command(s.as_str());

    // let p = embassy_stm32::init(Default::default());
    // let mut config = Config::default();
    // config.baudrate = 115200;

    // let mut usart = Uart::new(
    //     p.USART1, p.PA10, p.PA9, IrqsUART1, p.DMA1_CH4, p.DMA1_CH5, config,
    // )
    // .unwrap();
    // usart.write(b"Starting Echo\r\n").await.unwrap();
    // usart.write(s.as_bytes()).await.unwrap();

    // spawner.spawn(read_data_gps()).unwrap();
    // spawner.spawn(gps_main_task()).unwrap();

    //Test debug UART
    // task::debug_uart::show_data_debug("UART Debug").await;
    // spawner.spawn(task::debug_uart::show_data_hello()).unwrap();
    // task::debug_uart::show_data_debug("123").await;
    // let s: String<128> = String::new();
    // let _value = task::debug_uart::show_data_debug(debug_peripherals, &s);

    let mut msg: String<8> = String::new();

    for n in 0u32.. {
        // Obtain updated value from global context
        let shared = SHARED.lock().await;
        core::writeln!(&mut msg, "{:02}\r\n", *shared).unwrap();
        // // Transmit Message
        println!("{}", msg.as_str());
        msg.clear();

        let mut s: String<128> = String::new();
        core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();

        println!("{}", s.as_str());
        task::debug_uart::show_data_debug(&mut usart_debug, &s).await;
        // let _ = debug_peripherals.write(s.as_bytes()).await;

        Timer::after(Duration::from_millis(1000)).await;
    }
}
