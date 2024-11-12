#![no_std]
#![no_main]

// Neopixel Rainbow
// This only functions when the --release version is compiled. Using the debug
// version leads to slow pulse durations which results in a straight white LED
// output.
//
// // Needs to be compiled with --release for the timing to be correct

use panic_probe as _;

use framework_mcu_expansion_card_bsp as bsp;
use bsp::hal;
use bsp::pac;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::Hertz;
use hal::prelude::*;
use hal::sercom::spi;
use pac::{CorePeripherals, Peripherals};


use embedded_hal::digital::v2::OutputPin;

use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite
};
use ws2812_spi as ws2812;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let miso = pins.led_miso;
    let mosi = pins.led_in;
    let sck = pins.led_sck;

    let gclk = clocks.gclk0();

    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sck);
    let clock = clocks.sercom3_core(&gclk).unwrap();
    let spi = spi::Config::new(&peripherals.pm, peripherals.sercom3, pads, clock.freq())
        .baud(Hertz::MHz(3))
        .spi_mode(spi::MODE_0)
        .enable();

    let mut led_en = pins.led_en.into_push_pull_output();
    led_en.set_high().unwrap();

    let mut neopixel = ws2812::Ws2812::new(spi);

    // Loop through all of the available hue values (colors) to make a
    // rainbow effect from the onboard neopixel
    loop {
        for j in 0..255u8 {
            let colors = [hsv2rgb(Hsv {
                hue: j,
                sat: 255,
                val: 2,
            }); 1];
            neopixel.write(colors.iter().cloned()).unwrap();
            delay.delay_ms(5u8);
        }
    }
}
