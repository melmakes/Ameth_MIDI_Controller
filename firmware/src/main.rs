//! Blink the Seeeduino XIAO (SAMD21) onboard user LED via the `xiao_m0` BSP.
//!
//! Source: atsamd-rs/atsamd `boards/xiao_m0/examples/blink.rs`
//! (provenance 06710c7bc322eeae394426bd623b802118577a7a), verbatim.
//!
//! No pin substitution: the board crate owns the LED (`Led0` == PA17), which is
//! why this stack goes through the BSP rather than raw `atsamd-hal`.
#![no_main]
#![no_std]

extern crate panic_halt;

use hal::{clock::GenericClockController, delay::Delay, prelude::*};
use pac::{CorePeripherals, Peripherals};

use bsp::{entry, hal, pac};
use xiao_m0 as bsp;
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

   // spi.reconfigure()

    //list of fun colors
    let mut data: [RGB8; 7] = [RGB8::default(); 7];
    //list of all white
    let empty: [RGB8; 7] = [RGB8::default(); 7];
    let mut ws = Ws2812::new(spi);
    loop {
        data[0] = RGB8 {
            r: 246,
            g: 0,
            b: 0,
        };
        data[1] = RGB8 {
            r: 255,
            g: 140,
            b: 0,
        };
        data[2] = RGB8 {
            r: 255,
            g: 238,
            b: 0,
        };
        data[3] = RGB8 {
            r: 77,
            g: 233,
            b: 76,
        };
        data[4] = RGB8 {
            r: 55,
            g: 131,
            b: 255,
        };
        data[5] = RGB8 {
            r: 72,
            g: 21,
            b: 170,
        };
        data[6] = RGB8 {
            r: 151,
            g: 57,
            b: 223,
        };
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
