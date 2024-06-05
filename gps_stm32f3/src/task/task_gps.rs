use defmt::*;
use defmt_rtt as _;

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::Uart;
use embassy_stm32::peripherals;

#[embassy_executor::task]
pub async fn read_data_gps(power_pin: peripherals::PA4, mut gps_uart: Uart<'static, peripherals::USART2, peripherals::DMA1_CH7, peripherals::DMA1_CH6>) {
    info!("Task GPS");

    //Turn on power for module GPS
    // let _gps_pwr = Output::new(p.PA4, Level::High, Speed::VeryHigh);
    let _gps_pwr = Output::new(power_pin, Level::High, Speed::VeryHigh);

    let mut msg: [u8; 128] = [0; 128];
    loop {
        gps_uart.read(&mut msg).await.unwrap();
        
        if let Ok(message) = core::str::from_utf8(&msg) {
            println!("result: {}", message);
            // self::debug_uart::show_data_debug(message).await;
        }
    }
}
