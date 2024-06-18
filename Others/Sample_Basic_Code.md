### Sample Code
```
#[embassy_executor::task]
async fn gps_main_task() {
    let p = embassy_stm32::init(Default::default());
    // info!("Task GPS");

    //Turn on power for module GPS
    let _gps_pwr = Output::new(p.PA4, Level::High, Speed::VeryHigh);

    //GPS config
    let mut config_gps = Config::default();
    config_gps.baudrate = 9600;

    let mut usart_gps = Uart::new(
        p.USART2, p.PA3, p.PA2, IrqsUART2, p.DMA1_CH7, p.DMA1_CH6, config_gps,
    )
    .unwrap();

    //Debug UART config
    let mut config_debug = Config::default();
    config_debug.baudrate = 115200;

    let mut usart_debug = Uart::new(
        p.USART1, p.PA10, p.PA9, IrqsUART1, p.DMA1_CH4, p.DMA1_CH5, config_debug,
    )
    .unwrap();

    let mut msg: [u8; 128] = [0; 128];
    // let mut msg: Vec<u8, 128> = Default::default();

    usart_debug.write(b"GPS Task\r\n").await.unwrap();

    loop {
        usart_gps.read(&mut msg).await.unwrap();
        // usart_debug.write(&msg).await.unwrap();

        // Convert msg to a string, ignoring invalid UTF-8 sequences
        if let Ok(message) = core::str::from_utf8(&msg) {
            // Check the length of the message and return if it is 1024 characters long
            if message.len() == 128 {
                usart_debug.write(message.as_bytes()).await.unwrap();
                // Convert the message to heapless::String<1024>
                // let mut heapless_string: String<128> = String::new();
                // if heapless_string.push_str(message).is_ok() {
                    // return Some(heapless_string);
                    // let _ = write_command(heapless_string);
                    // usart_debug.write(message.as_bytes()).await.unwrap();
                // }
                // let _ = write_command(message);
            }
        }
    }
    
}
```








