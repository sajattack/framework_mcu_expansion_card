#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;

pub use hal::prelude::*;

pub use hal::pac;

use hal::{
    sercom::{
        i2c,
        spi,
        uart::{self, BaudMode, Oversampling, Pads},
    },
    clock::GenericClockController,
    time::Hertz,
    usb::{
        UsbBus,
        usb_device::bus::UsbBusAllocator,
    },
};

use pac::{Sercom0, Sercom1, Sercom2};

atsamd_hal::bsp_pins!(
    PA00 {
        name: d0,
    },
    PA01 {
        name: d1,
    },
    PA02 {
        name: a0,
    },
    PA03 {
        name: a1,
    },
    PA04 {
        name: a2,
    },
    PA05 {
        name: a3,
    },
    PA06 {
        name: a6,
        aliases: {
            AlternateD: UartTx,
        }
    },
    PA07 {
        name: a7,
        aliases: {
            AlternateD: UartRx,
        }
    },
    PA08 {
        name: flash_cs,
    },
    PA09 {
        name: cipo,
        aliases: {
            AlternateD: Miso
        }
    },
    PA10 {
        name: copi,
        aliases: {
            AlternateD: Mosi
        }
    },
    PA11 {
        name: sck,
        aliases: {
            AlternateD: Sclk
        }
    },
    PA14 {
        name: d2,
    }
    PA15 {
        name: ws2812_en,
    },
    PA16 {
        name: sda,
        aliases: {
            AlternateC: Sda
        }
    },
    PA17 {
        name: scl,
        aliases: {
            AlternateC: Scl
        }
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
        aliases: {
            AlternateG: UsbDm
        }
    },
    PA25 {
        name: usb_dp,
        aliases: {
            AlternateG: UsbDp
        }
    },
    PA27 {
        name: d3,
    },
    PA28 {
        name: d4,
    }
);

hal::bsp_peripherals!(
    Sercom0 { UartSercom }
    Sercom1 { I2cSercom }
    Sercom2 { SpiSercom }
);

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<I2cSercom, Sda, Scl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](ehal::blocking::i2c::Read),
/// [`Write`](ehal::blocking::i2c::Write) and
/// [`WriteRead`](ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: I2cSercom,
    pm: &mut pac::Pm,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom1_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(pm, sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<UartSercom, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: UartSercom,
    pm: &mut pac::Pm,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<SpiSercom, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up the SPI SERCOM and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: Hertz,
    sercom: SpiSercom,
    pm: &mut pac::Pm,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(pm, sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

pub fn usb_allocator(
    usb: pac::Usb,
    clocks: &mut GenericClockController,
    pm: &mut pac::Pm,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
