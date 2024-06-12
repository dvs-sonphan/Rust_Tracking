
use defmt::*;
use defmt_rtt as _;

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals;
use embassy_stm32::usart::Uart;

use alloc::vec::Vec;
use embassy_time::{Duration, Timer};
// use alloc::vec;
// extern crate nmea_parser;
use nmea_parser::*;
// use heapless::Vec; // fixed capacity `std::Vec`

extern crate chrono;
use chrono::{DateTime, Utc};

#[embassy_executor::task]
pub async fn read_data_gps(
    power_pin: peripherals::PA4,
    mut gps_uart: Uart<'static, peripherals::USART2, peripherals::DMA1_CH7, peripherals::DMA1_CH6>,
) {
    info!("Task GPS");

    //Turn on power for module GPS
    // let _gps_pwr = Output::new(p.PA4, Level::High, Speed::VeryHigh);
    let _gps_pwr = Output::new(power_pin, Level::High, Speed::VeryHigh);

    let mut msg: [u8; 256] = [0; 256];
    loop {
        gps_uart.read(&mut msg).await.unwrap();

        if let Ok(message) = core::str::from_utf8(&msg) {
            println!("result: {}", message);
            // self::debug_uart::show_data_debug(message).await;

            let mut vec = Vec::new();
            for line in message.lines() {
                // println!("{}", line);
                let _vec_message = vec.push(line);
            }

            // let vec_message = vec.extend_from_slice(&[message]);
            // let _vec_message = vec.push(&message);
            // println!("vec at {:?}", vec.as_slice());

            parse_data_gps(vec.clone()).await;
            // parse_data_gps().await;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

// #[cfg(any(feature = "std", feature = "alloc"))]
// #[embassy_executor::task]
pub async fn parse_data_gps(buf: Vec<&str>) {
    // pub async fn parse_data_gps() {
    // Create a fixed-capacity vector using heapless
    // let mut vec: Vec<&str, 128> = Vec::new();
    // vec_data.push(1).unwrap();
    // vec.push(1).expect("Failed to push to vec");

    // println!("Parse Data GPS");

    // Create parser and define sample sentences
    let mut parser = NmeaParser::new();
    // let buf = alloc::vec![
    //     "!AIVDM,1,1,,A,H42O55i18tMET00000000000000,2*6D",
    //     "!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40",
    //     // "$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*56",
    //     "GNRMC,063453.083,V,,,,,0.00,0.00,120624,,,N*5C\r\n$GNVTG,0.00,T,,M,0.00,N,0.00,K,N*2C\r\n$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*56\r\n$GPG"
    // ];

    // Parse the sentences and print some fields of the messages
    for sentence in buf {
        // println!("{}", sentence);
        match parser.parse_sentence(sentence) {
            // core::prelude::v1::Ok(ParsedMessage::VesselDynamicData(vdd)) => {
            //     println!("MMSI:    {}", vdd.mmsi);
            //     println!("Speed:   {:.1} kts", vdd.sog_knots.unwrap());
            //     println!("Heading: {}°", vdd.heading_true.unwrap());
            //     println!("");
            // }
            // core::prelude::v1::Ok(ParsedMessage::VesselStaticData(vsd)) => {
            //     println!("MMSI:  {}", vsd.mmsi);
            //     println!("Flag:  {}", vsd.country().unwrap());
            //     println!("Name:  {}", vsd.name.unwrap());
            //     println!("Type:  {}", vsd.ship_type);
            //     println!("");
            // }
            core::prelude::v1::Ok(ParsedMessage::Gga(gga)) => {
                // println!("Source:    {}", gga.source);
                println!("Latitude:  {}°", gga.latitude.unwrap());
                println!("Longitude: {}°", gga.longitude.unwrap());
                println!("");
            }
            core::prelude::v1::Ok(ParsedMessage::Rmc(rmc)) => {
                // println!("Source:  {}", rmc.source);
                println!("Speed:   {} kts", rmc.sog_knots.unwrap());
                println!("Bearing: {}°", rmc.bearing.unwrap()); //Góc
                let mut _date_time = rmc.timestamp;
                // println!("Time: {}", Some(date_time).unwrap());
                println!("");
            }
            _ => {}
        }
    }
}
