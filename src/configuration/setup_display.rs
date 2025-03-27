use display_interface_spi::SPIInterfaceNoCS;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::{Gpio2, Gpio4, PinDriver, Pins};
use esp_idf_hal::prelude::{MegaHertz};
use esp_idf_hal::spi::config::{Config, DriverConfig};
use esp_idf_hal::spi::{Dma, SpiDeviceDriver, SpiDriver, SPI2};
use mipidsi::{Builder, Display};
use mipidsi::models::ILI9341Rgb565;

pub fn setup_display(pins : Pins, spi : SPI2) ->
Result<Display<SPIInterfaceNoCS<SpiDeviceDriver<'static, SpiDriver<'static>>, PinDriver<'static, Gpio2, gpio::Output>>, ILI9341Rgb565, PinDriver<'static, Gpio4, gpio::Output>>, String>
{
    // Reset
    let rst = gpio::PinDriver::output(pins.gpio4).map_err(|_| String::from("Cannot init GPIO4"))?;
    // Data Command control pin
    let dc = gpio::PinDriver::output(pins.gpio2).map_err(|_| String::from("Cannot init GPIO2"))?;

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
        ).map_err(|_| String::from("Cannot init SPI Interface"))?,
        dc,
    );

    // Initialize the display with the ILI9341 driver
    let display = Builder::ili9341_rgb565(di)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .with_orientation(mipidsi::options::Orientation::Landscape(false))  // Mirror on text
        .init(&mut delay, Some(rst))
        .map_err(|_| String::from("Cannot init Display"))?;


    // Pin 27, Backlight
    let mut bl = gpio::PinDriver::output(pins.gpio27).map_err(|_| String::from("Cannot init GPIO27"))?;
    // Turn on backlight
    bl.set_high().map_err(|_| String::from("Cannot set pin27 high"))?;
    // Force the GPIO to hold its high state
    core::mem::forget(bl);

    Ok(display)
}