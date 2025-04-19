use std::error::Error;
use display_interface_spi::SPIInterfaceNoCS;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::{Gpio2, Gpio4, PinDriver, Pins};
use esp_idf_hal::prelude::{MegaHertz};
use esp_idf_hal::spi::config::{Config, DriverConfig};
use esp_idf_hal::spi::{Dma, SpiDeviceDriver, SpiDriver, SPI2};
use mipidsi::{Builder, Display};
use mipidsi::models::ILI9341Rgb565;

pub type SpiDisplayInterface = SPIInterfaceNoCS<
    SpiDeviceDriver<'static, SpiDriver<'static>>,
    PinDriver<'static, Gpio2, gpio::Output>
>;

pub type DisplayDriver = Display<SpiDisplayInterface, ILI9341Rgb565, PinDriver<'static, Gpio4, gpio::Output>>;

pub fn setup_display(pins : Pins, spi : SPI2) -> anyhow::Result<DisplayDriver>
{
    // Reset
    let rst = gpio::PinDriver::output(pins.gpio4)?;
    // Data Command control pin
    let dc = gpio::PinDriver::output(pins.gpio2)?;

    // Espressif built-in delay provider for small delays
    let mut delay = Ets;

    // Pin 14, Serial Clock
    let sclk = pins.gpio14;

    // Pin 13, MOSI, Master Out Slave In
    let sdo = pins.gpio13;
    // Pin 12, MISO, Master In Slave Out (unused in this example)
    let sdi = pins.gpio12;

    let cs = pins.gpio15;

    // SPI interface with no chip select
    let di = SPIInterfaceNoCS::new(
        SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Some(sdi),
            Some(cs),
            &DriverConfig::new().dma(Dma::Disabled),
            &Config::new().baudrate(MegaHertz(40).into()),
        )?,
        dc,
    );

    // Initialize the display with the ILI9341 driver
    let display = Builder::ili9341_rgb565(di)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .with_orientation(mipidsi::options::Orientation::Landscape(false))  // Mirror on text
        .init(&mut delay, Some(rst)).map_err(|e| SimpleError::new("ESP Init"))?;


    // Pin 27, Backlight
    let mut bl = gpio::PinDriver::output(pins.gpio27)?;
    // Turn on backlight
    bl.set_high()?;
    // Force the GPIO to hold its high state
    core::mem::forget(bl);

    Ok(display)
}

#[derive(Debug)]
pub struct SimpleError {
    description: &'static str,
}

impl SimpleError {
    pub fn new(description: &'static str) -> Self {
        Self { description }
    }
}

impl core::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SimpleError {}