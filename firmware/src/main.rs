#![no_main]
#![no_std]

extern crate panic_halt;

use hal::{ehal::digital, clock::GenericClockController, delay::Delay, prelude::*};
use pac::{CorePeripherals, Peripherals};

use bsp::{entry, hal, pac};
use xiao_m0 as bsp;
use hal::fugit::RateExtU32;

//use ws2812_spi as ws2812;

//use crate::ws2812::Ws2812;
//use smart_leds::SmartLedsWrite;
//use smart_leds_trait::RGB8;
use ws2812_rs::{WS2812SPI, SendColorBySPI, Color};

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

    //name simplification for our pins
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let (miso,mosi,sclk) = (pins.a9, pins.a10, pins.a8 );

    let spi = bsp::spi_master(
        &mut clocks,
        3.MHz(),
        peripherals.SERCOM0,
        &mut pm,
        sclk,
        mosi,
        miso,
    );

    //list of fun colors
    //let mut data: [RGB8; 7] = [RGB8::default(); 7];
    //list of all white
    //let empty: [RGB8; 7] = [RGB8::default(); 7];
    //let mut ws = Ws2812::new(spi);
    //let led_pin = pins.a10.into_push_pull_output();

    let mut leds = WS2812SPI::new(spi); 
    let data: [Color; 7] = [
        Color::red(),
        Color::orange(),
        Color::yellow(),
        Color::green(),
        Color::cyan(),
        Color::blue(),
        Color::purple()];

    loop {
        
        //writes green?
        
        leds.write_colors(data, 3_000_000).unwrap();
        delay.delay_ms(1000_u32);
        //writes white?
        leds.write_colors([Color::white(); 7], 3_000_000).unwrap();
        delay.delay_ms(1000_u32);
    }
}
