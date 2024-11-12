#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;

pub use hal::prelude::*;

pub use hal::pac;

atsamd_hal::bsp_pins!(
    PA00 {
        name: D0,
    },
    PA01 {
        name: D1,
    },
    PA02 {
        name: A0,
    },
    PA03 {
        name: A1,
    },
    PA04 {
        name: A2,
    },
    PA05 {
        name: A3,
    },
    PA06 {
        name: A6,
        aliases: {
            AlternateD: Tx,
        }
    },
    PA07 {
        name: A7,
        aliases: {
            AlternateD: Rx,
        }
    },
    PA08 {
        name: flash_cs,
    },
    PA09 {
        name: cipo,
    },
    PA10 {
        name: copi,
    },
    PA11 {
        name: sck,
    },
    PA14 {
        name: d2,
    }
    PA15 {
        name: ws2812_en,
    },
    PA16 {
        name: sda
    },
    PA17 {
        name: scl,
    },
    PA18 {
        name: ws2812_in,
    },
    PA19 {
        name: flash_cipo,
    },
    PA22 {
        name: flash_copi,
    },
    PA23 {
        name: flash_clk,
    },
    PA24 {
        name: usb_dn,
    },
    PA25 {
        name: usb_dp,
    },
    PA27 {
        name: d3,
    },
    PA28 {
        name: d4,
    }
);
