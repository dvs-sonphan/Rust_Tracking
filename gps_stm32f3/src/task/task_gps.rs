use core::fmt::*;

use defmt::*;
use defmt_rtt as _;

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals;
use embassy_stm32::usart::Uart;

use alloc::vec::Vec;
use embassy_time::{Duration, Timer};
use nmea_parser::*;

// extern crate chrono;
// use chrono::{DateTime, Utc};


// #[derive(Debug)]
// enum GPSUpdate {
//     // DateTime(DateTime<Utc>),
//     Lat(f64),
//     Long(f64),
//     Sat(u8),
//     Speed(f64),
// }

#[derive(Clone, Copy, Debug, Format)]
struct GPSData {
    // date_time:DateTime<Utc>,
    lat: f64,
    long: f64,
    sat: u8,
    speed: f64,
}

impl GPSData {
    fn update_lat(&mut self, new_lat: f64) {
        self.lat = new_lat;
    }

    fn update_long(&mut self, new_long: f64) {
        self.long = new_long;
    }

    fn update_sat(&mut self, new_sat: u8) {
        self.sat = new_sat;
    }

    fn update_speed(&mut self, new_speed: f64) {
        self.speed = new_speed;
    }
}

// impl GPSData {
//     fn apply_update(&mut self, update: GPSUpdate) {
//         match update {
//             GPSUpdate::Lat(new_lat) => self.lat = new_lat,
//             GPSUpdate::Long(new_long) => self.long = new_long,
//             GPSUpdate::Sat(new_sat) => self.sat = new_sat,
//             GPSUpdate::Speed(new_speed) => self.speed = new_speed,
//         }
//     }
// }

// #[embassy_executor::task]
pub async fn read_data_gps(
    mut gps_uart: Uart<'static, peripherals::USART2, peripherals::DMA1_CH7, peripherals::DMA1_CH6>,
) {
    let mut msg: [u8; 256] = [0; 256];
    loop {
        gps_uart.read(&mut msg).await.unwrap();

        if let Ok(message) = core::str::from_utf8(&msg) {
            println!("result: {}", message);

            let mut vec = Vec::new();
            for line in message.lines() {
                // println!("{}", line);
                let _vec_message = vec.push(line);
            }
            // println!("vec at {:?}", vec.as_slice());
            parse_data_gps(vec.clone()).await;

            //clean array
            for elem in msg.iter_mut() {
                *elem = 0;
            }
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

pub async fn parse_data_gps(buf: Vec<&str>) {
    info!("Parse Data GPS");
    let mut gps_data = GPSData {
        lat: 0.0,
        long: 0.0,
        sat: 0,
        speed: 0.0,
    };

    // Create parser and define sample sentences
    let mut parser = NmeaParser::new();

    // Parse the sentences and print some fields of the messages
    for sentence in buf {
        // println!("{}", sentence);
        match parser.parse_sentence(sentence) {
            core::prelude::v1::Ok(ParsedMessage::Gga(gga)) => {
                // let gga_lat = GPSUpdate::Lat(gga.latitude.unwrap());
                // let gga_long = GPSUpdate::Long(gga.longitude.unwrap());
                // let gga_sate = GPSUpdate::Sat(gga.satellite_count.expect("Satellite Errors"));

                // Updating GPS data
                gps_data.update_lat(gga.latitude.unwrap());
                gps_data.update_long(gga.longitude.unwrap());
                gps_data.update_sat(gga.satellite_count.expect("Satellite Errors"));

                println!("");
            }

            core::prelude::v1::Ok(ParsedMessage::Rmc(rmc)) => {
                // let Some(date_time_data) = rmc.timestamp else {
                //     todo!()
                // };

                // println!("Source:  {}", rmc.source);
                // println!("Speed:   {} kts", rmc.sog_knots.unwrap());
                // println!("Bearing: {}°", rmc.bearing.unwrap()); //Góc

                // let rmc_speed = GPSUpdate::Speed(rmc.sog_knots.unwrap());
                // let rmc_long = GPSUpdate::Long(rmc.bearing.unwrap());

                gps_data.update_speed(rmc.sog_knots.unwrap());

                println!("");
            }
            _ => {}
        }

        //show data gps
        println!("Updated GPS Data: {:?}", gps_data);
    }
}

#[embassy_executor::task]
pub async fn main_task_gps(
    power_pin: peripherals::PA4,
    gps_uart: Uart<'static, peripherals::USART2, peripherals::DMA1_CH7, peripherals::DMA1_CH6>,
) {
    info!("Task GPS");

    //Turn on power for module GPS
    // let _gps_pwr = Output::new(p.PA4, Level::High, Speed::VeryHigh);
    let _gps_pwr = Output::new(power_pin, Level::High, Speed::VeryHigh);

    //Read data GPS
    read_data_gps(gps_uart).await;

    //show data gps
    // let mut gps_data = GPSData { lat: todo!(), long: todo!(), sat: todo!(), speed: todo!() };
    // println!("Updated GPS Data: {:?}", gps_data);
}
