use core::fmt::Write;

use defmt::*;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

// #[embassy_executor::task]
pub async fn debug_uart(text: &str) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let config = Config::default();
    let mut usart = Uart::new(
        p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH4, p.DMA1_CH5, config,
    )
    .unwrap();

    let mut s: String<128> = String::new();
    core::write!(&mut s, "{}\r\n", text).unwrap();

    unwrap!(usart.write(s.as_bytes()).await);
    info!("wrote DMA");
}
