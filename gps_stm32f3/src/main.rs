#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(alloc_error_handler)]

extern crate alloc;
// use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;                        
                                                        
// this is the allocator the application will use       
#[global_allocator]                                     
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();   
                                                        
const HEAP_SIZE: usize = 1024; // in bytes              

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

use crate::task::task_gps;

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
    spawner.spawn(task::task_gps::main_task_gps(p.PA4, usart_gps)).unwrap();

    //************************************************* */
    // let mut gps_data = task_gps()
    // let mut msg: String<128> = String::new();

    // for n in 0u32.. {
    //     core::write!(&mut msg, "Hello DMA World {}!\r\n", n).unwrap();

    //     println!("{}", msg.as_str());
    //     task::debug_uart::show_data_debug(&mut usart_debug, &msg).await;
    //     msg.clear();

    //     Timer::after(Duration::from_millis(1000)).await;
    // }
}

// define what happens in an Out Of Memory (OOM) condition     
#[alloc_error_handler]                                         
fn alloc_error(_layout: core::alloc::Layout) -> ! {            
    loop {}                                                    
}
