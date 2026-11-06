
#![no_main]
#![no_std]

extern crate panic_halt;

use hal::{clock::GenericClockController, delay::Delay, prelude::*};
use bsp::{entry, hal, pac};
use hal::timer::TimerCounter;
use pac::{CorePeripherals, Peripherals};
use xiao_m0::{self as bsp, hal::sercom::spi};
use hal::fugit::RateExtU32;

use ws2812_spi as ws2812;

use crate::ws2812::Ws2812;
use smart_leds::SmartLedsWrite;
use smart_leds_trait::RGB8;

#[entry]
fn main() -> ! {   
   //basic setup for embedded stuff
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    //getting peripherals
    let mut pm= peripherals.PM;

    //getting pins
    let pins = bsp::Pins::new(peripherals.PORT);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    //easier names than referencing pins every time
    let (miso,mosi,sclk) = (pins.a9, pins.a10, pins.a8 );
    
    //spi peripheral instantiation
    let mut spi = bsp::spi_master(
        &mut clocks,
        2.MHz(),
        peripherals.SERCOM0,
        &mut pm,
        sclk,
        mosi,
        miso,
    );

    //post-hoc reconfiguring
    let mut cfg = spi.disable();
    cfg.set_spi_mode(spi::MODE_0);
    spi = cfg.enable();
    

    //list of fun colors
    let mut data: [RGB8;7] = [RGB8::default();7];
    data[0] = RGB8{r:255,g:0,b:0};//red
    data[1] = RGB8{r:255,g:127,b:0};//orange
    data[2] = RGB8{r:255,g:255,b:0};//yellow
    data[3] = RGB8{r:0,g:255,b:0};//green
    data[4] = RGB8{r:0,g:0,b:255};//blue
    data[5] = RGB8{r:75,g:0,b:130};//indigo
    data[6] = RGB8{r:148,g:0,b:211};//violet
    
    //list of all white
    let empty: [RGB8;7] = [RGB8::default();7];

    //let null: [RGB8;1] = [data[0]];
    
    let mut ws = Ws2812::new(spi);
    //ws.mode = 
    loop {
        
        //writes our designated colors?
        ws.write(data.iter().cloned())
            .expect("we need to be able to iterate");
        delay.delay_ms(1000_u32);
        //writes white?
        ws.write(empty.iter().cloned())
            .expect("we need to be able to iterate after a delay");
        delay.delay_ms(1000_u32);
    }
}
