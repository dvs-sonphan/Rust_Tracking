// main.rs
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod task;
use core::fmt::Write;

use defmt::*;
use defmt_rtt as _;

use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use heapless::String;
use embassy_time::{Duration, Timer};
use panic_probe as _;

bind_interrupts!(struct IrqsUART1 {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

bind_interrupts!(struct IrqsUART2 {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("GPS Tracking");
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
    task::debug_uart::show_data_debug(&mut usart_debug, "GPS Task\r\n").await;
    task::debug_uart::show_data_debug(&mut usart_debug, "Test Multi-Task\r\n").await;
    //GPS Task
    spawner.spawn(task::task_gps::read_data_gps(p.PA4, usart_gps)).unwrap();

    //************************************************* */
    let mut msg: String<128> = String::new();

    for n in 0u32.. {
        core::write!(&mut msg, "Hello DMA World {}!\r\n", n).unwrap();

        println!("{}", msg.as_str());
        task::debug_uart::show_data_debug(&mut usart_debug, &msg).await;
        msg.clear();

        Timer::after(Duration::from_millis(1000)).await;
    }
}
