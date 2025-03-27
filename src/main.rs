mod configuration;
mod utils;

use embedded_graphics::{
    mono_font::{MonoTextStyle},
    prelude::*,
    text::Text,
    geometry::Point,
};
use esp_idf_svc::hal::{prelude::Peripherals};
use std::error::Error;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb565;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use crate::configuration::setup_display::setup_display;
use crate::configuration::setup_wifi::{connect_wifi, get_request};
use embedded_svc::{
    http::{client::Client as HttpClient},
};
use crate::utils::always_same::AlwaysSame;
use esp_idf_hal::delay::Ets;
use esp_idf_sys::esp_task_wdt_reset;

const SSID: &str = std::env!("SSID");
const PASSWORD: &str = std::env!("PASSWORD");

fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    //Init Logging
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");

    //Init Peripherals
    let peripherals = Peripherals::take().unwrap();

    //Display
    let mut display = setup_display(peripherals.pins, peripherals.spi2)?;

    let pixels = AlwaysSame {value: Rgb565::BLACK};
    display.set_pixels( 0,0,540,300, pixels.into_iter().take(540*300) )
        .map_err(|_| Box::<dyn Error>::from("draw world"))?;
    log::info!("Cleared Display");

    //Wi-Fi
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;
    let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
    log::info!("Connected WiFi");

    // GET
    let requestString = get_request(&mut client)?;
    log::info!("Got request");

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
    Text::new(&requestString, Point::new(0, 0), text_style)
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw world"))?;


    Ok(())
}