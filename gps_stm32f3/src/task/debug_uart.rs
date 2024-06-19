use defmt::*;
use defmt_rtt as _;

use embassy_stm32::peripherals::USART1;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::Peripherals;
use embassy_stm32::peripherals::{DMA1_CH4, DMA1_CH5};

use crate::IrqsUART1;

#[allow(dead_code)]
pub fn init_peripheral(p: Peripherals) -> Uart<'static, USART1, DMA1_CH4, DMA1_CH5> {
    let mut config_debug = Config::default();
    config_debug.baudrate = 115200;

    let uart = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        IrqsUART1,
        p.DMA1_CH4,
        p.DMA1_CH5,
        config_debug,
    )
    .unwrap();

    uart
}

pub async fn show_data_debug(uart: &mut Uart<'static, USART1, DMA1_CH4, DMA1_CH5>, message: &str) {
    // info!("Debug Command");

    uart.write(message.as_bytes()).await.unwrap();
    // uart.blocking_write(message.as_bytes()).unwrap();
}

#[allow(dead_code)]
#[embassy_executor::task]
pub async fn read_command() {
    let p = embassy_stm32::init(Default::default());
    info!("Read Command");

    let config = Config::default();
    let mut usart = Uart::new(
        p.USART1, p.PA10, p.PA9, IrqsUART1, p.DMA1_CH4, p.DMA1_CH5, config,
    )
    .unwrap();

    let mut msg: [u8; 8] = [0; 8];

    loop {
        usart.read(&mut msg).await.unwrap();
        usart.write(&msg).await.unwrap();
    }
}
