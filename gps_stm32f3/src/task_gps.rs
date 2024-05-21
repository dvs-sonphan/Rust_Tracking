use core::fmt::Write;

use defmt::*;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::task]
pub async fn read_data_gps() {
    let p = embassy_stm32::init(Default::default());
    info!("Task GPS");

    let config = Config::default();
    let mut usart = Uart::new(
        // p.USART2, p.PA3, p.PA2, Irqs
        // p.USART2, p.PA3, p.PA2, Irqs, p.DMA1_CH4, p.DMA1_CH5, config,
    )
    .unwrap();

    let mut s: String<128> = String::new();
    core::write!(&mut s, "{}\r\n", text).unwrap();

    unwrap!(usart.write(s.as_bytes()).await);
    info!("wrote DMA");
}
