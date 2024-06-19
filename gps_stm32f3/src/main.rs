#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(alloc_error_handler)]

// use core::fmt::Write;
// use heapless::String;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

//------------------------- Define Macro Alloc ------------------------------------
extern crate alloc;
// use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

//----------------------- Define Module Tasks ---------------------------------
mod task;
use crate::task::task_gps::GPSData;

//----------------------- Define embassy framwork -----------------------------
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_time::{Duration, Timer};
use embassy_stm32::wdg::IndependentWatchdog;

//----------------------- Define Channel use embassy framwork ----------------
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;

static CHANNEL: Channel<ThreadModeRawMutex, GPSData, 64> = Channel::new();

//------------------------ Define Others --------------------------------------
bind_interrupts!(struct IrqsUART1 {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

bind_interrupts!(struct IrqsUART2 {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    info!("GPS Tracking");
    let p = embassy_stm32::init(Default::default());

    //************ Define Watchdog-Timer******************** */
    let mut wdt = IndependentWatchdog::new(p.IWDG, 20_000_000);
    wdt.unleash();

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

    let gps_data = GPSData::new();

    //******************* Tasks ***************************
    task::debug_uart::show_data_debug(&mut usart_debug, "GPS Tracking\r\n").await;
    //GPS Task
    spawner
        .spawn(task::task_gps::main_task_gps(
            p.PA4,
            usart_gps,
            gps_data,
            CHANNEL.sender(),
        ))
        .unwrap();

    //************************************************* */
    for _n in 0u32.. {
        // Receive updated GPS data from the task
        let gps_data = CHANNEL.receiver().receive().await;

        info!("GET Data");
        // Access data using getter methods
        println!("Latitude: {}", gps_data.get_lat());
        println!("Longitude: {}", gps_data.get_long());
        println!("Satellites: {}", gps_data.get_sat());
        println!("Speed: {}", gps_data.get_speed());

        //output to debug serial
        // let mut gps_lat: String<64> = String::new();
        // core::write!(&mut gps_lat, "Latitude: {}!\r\n", gps_data.get_lat().to_string()).unwrap();
        // let mut gps_long: String<64> = String::new();
        // core::write!(&mut gps_long, "Longitude: {}!\r\n", gps_data.get_lat().to_string()).unwrap();
        // let mut gps_sat: String<8> = String::new();
        // core::write!(&mut gps_sat, "Satellites: {}!\r\n", gps_data.get_lat().to_string()).unwrap();
        // let mut gps_speed: String<64> = String::new();
        // core::write!(&mut gps_speed, "Speed: {}!\r\n", gps_data.get_lat().to_string()).unwrap();

        // task::debug_uart::show_data_debug(&mut usart_debug, &gps_lat.as_str()).await;
        // task::debug_uart::show_data_debug(&mut usart_debug, &gps_long.as_str()).await;
        // task::debug_uart::show_data_debug(&mut usart_debug, &gps_sat.as_str()).await;
        // task::debug_uart::show_data_debug(&mut usart_debug, &gps_speed.as_str()).await;

        Timer::after(Duration::from_millis(1000)).await;
    }

    // Reset WDT
    wdt.pet();
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    loop {}
}
