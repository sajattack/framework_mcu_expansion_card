#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;

pub use hal::prelude::*;

pub use hal::pac;

atsamd_hal::bsp_pins!(
    PA15 {
        name: led_en,
    },
    PA18 {
        name: led_in,
    },
    PA19 {
        name: led_sck,
    }
    PA17 {
        name: led_miso,
    },
);
