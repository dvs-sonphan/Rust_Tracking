use core::fmt::*;

use defmt::*;
use defmt_rtt as _;
use alloc::vec::Vec;

use nmea_parser::*;

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals;
use embassy_stm32::usart::Uart;
use embassy_time::{Duration, Timer};

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Sender;

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
pub struct GPSData {
    // date_time:DateTime<Utc>,
    lat: f64,
    long: f64,
    sat: u8,
    speed: f64,
}

impl GPSData {
    pub fn new() -> Self {
        GPSData {
            lat: 0.0,
            long: 0.0,
            sat: 0,
            speed: 0.0,
        }
    }

    // Update GPS Data
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

    // Get GPS Data
    pub fn get_lat(&self) -> f64 {
        self.lat
    }

    pub fn get_long(&self) -> f64 {
        self.long
    }

    pub fn get_sat(&self) -> u8 {
        self.sat
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }
}

// read_data_gps loop forever read data GPS
pub async fn read_data_gps(
    mut gps_uart: Uart<'static, peripherals::USART2, peripherals::DMA1_CH7, peripherals::DMA1_CH6>,
    gps_data: GPSData,
    sender: Sender<'static, ThreadModeRawMutex, GPSData, 64>
) {
    info!("Read Data GPS");
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
            parse_data_gps(vec.clone(), gps_data, sender).await;

            //clean array
            for elem in msg.iter_mut() {
                *elem = 0;
            }
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

pub async fn parse_data_gps(buf: Vec<&str>, mut gps_data: GPSData, sender: Sender<'static, ThreadModeRawMutex, GPSData, 64>) {
    info!("Parse Data GPS");

    // Create parser and define sample sentences
    let mut parser = NmeaParser::new();

    // Parse the sentences and print some fields of the messages
    for sentence in buf {
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

                gps_data.update_speed(rmc.sog_knots.unwrap());

                println!("");
            }
            _ => {}
        }

        //show data gps
        // println!("Updated GPS Data: {:?}", gps_data);

        // Send updated data to the main thread
        sender.send(gps_data).await;
    }
}

#[embassy_executor::task]
pub async fn main_task_gps(
    power_pin: peripherals::PA4,
    gps_uart: Uart<'static, peripherals::USART2, peripherals::DMA1_CH7, peripherals::DMA1_CH6>,
    gps_data: GPSData,
    sender: Sender<'static, ThreadModeRawMutex, GPSData, 64>,
) {
    info!("Task GPS");

    //Turn on power for module GPS
    let _gps_pwr = Output::new(power_pin, Level::High, Speed::VeryHigh);

    //show data gps
    // println!("Updated GPS Data: {:?}", gps_data);

    //Read data GPS
    read_data_gps(gps_uart, gps_data, sender).await;
}
