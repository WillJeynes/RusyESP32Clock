extern crate core;

mod configuration;
mod utils;
mod font_bytes;

use embedded_graphics::{
    mono_font::{MonoTextStyle},
    prelude::*,
    text::Text,
    geometry::Point,
};
use esp_idf_svc::hal::{prelude::Peripherals};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::TZ_VARIANTS;
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::pixelcolor::{Rgb565, Rgb888};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use crate::configuration::setup_display::{setup_display, DisplayDriver};
use crate::configuration::setup_wifi::{connect_wifi, get_request, get_request_raw};
use embedded_svc::{
    http::{client::Client as HttpClient},
};
use embedded_svc::http::status::INFO;
use crate::utils::always_same::AlwaysSame;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::{Gpio2, Gpio4, PinDriver};
use esp_idf_hal::modem::Modem;
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};
use esp_idf_sys::{esp_task_wdt_reset, esp_timer_get_time};
use mipidsi::Display;
use mipidsi::models::ILI9341Rgb565;
use tinybmp::Bmp;
use crate::utils::screen_utils::{clear_screen, draw_text};
use crate::utils::simple_error::ContextExt;

const SSID: &str = std::env!("SSID");
const PASSWORD: &str = std::env!("PASSWORD");

const BASEURL: &str = std::env!("BASEURL");

const DEBUG: &str = std::env!("DEBUG");

const LOCATION: &str = std::env!("LOCATION");

include!(concat!(env!("OUT_DIR"), "/font_bytes.rs"));

fn main() -> anyhow::Result<()> {
    let is_debug: bool = DEBUG == "TRUE";

    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    //Init Logging
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");

    //Init Peripherals
    let peripherals = Peripherals::take()?;

    //Display
    let mut display = setup_display(peripherals.pins, peripherals.spi2)?;

    clear_screen(&mut display, if (is_debug) { Rgb565::BLUE } else { Rgb565::BLACK})?;

    draw_text(&mut display, "Loading...", Point::new(10, 10), Rgb565::WHITE)?;

    log::info!("Cleared Display");

    match run(peripherals.modem, &mut display) {
        Ok(vak) => log::info!("Run successfully"),
        Err(error) => draw_err(error, &mut display)
    }

    Ok(())
}

fn draw_err(err : anyhow::Error, display : &mut DisplayDriver) {
    draw_text(display, &format!("Ex: {}", err), Point::new(10, 50), Rgb565::RED).unwrap();
}
fn run(modem: Modem, mut display: &mut DisplayDriver) -> anyhow::Result<()> {
    let is_debug: bool = DEBUG == "TRUE";

    //Wi-Fi
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    for attempt in 1..=3 {
        match connect_wifi(&mut wifi) {
            Ok(_) => {
                log::info!("Connected to Wi-Fi on attempt {}", attempt);
            }
            Err(e) => {
                log::error!("Failed to connect (attempt {}): {}", attempt, e);
                if attempt < 3 {
                    sleep(Duration::from_secs(2));
                }
            }
        }
    }

    let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
    log::info!("Connected WiFi");


    // GET
    let request_string = get_request(&mut client, format!("{}/Time/GetCurrentTime", BASEURL))?;
    let request_clock = request_string.parse::<i64>()?;
    let request_millis = unsafe {esp_timer_get_time()} / 1000;
    log::info!("Got Time Request {} at {}",request_string,  request_millis);

    let mut current_date = String::from("");
    let mut current_time = String::from("aaaaaa");

    let mut last_updated_at_a = 0;
    let mut last_updated_at_b = 0;

    //Time-zoning
    let found = TZ_VARIANTS.iter().find(|v| {
        v.name() == LOCATION
    }).unwrap();

    let font_bmps = FONT_BYTES.map(|data|  Bmp::<Rgb888>::from_slice(data).unwrap());

    clear_screen(&mut display, if (is_debug) { Rgb565::BLUE } else { Rgb565::BLACK})?;

    loop {
        let current_millis = unsafe {esp_timer_get_time()} / 1000;
        let difference = current_millis - request_millis;
        log::info!("Current Time difference: {} ms", difference);
        let current_millis = request_clock + difference;
        log::info!("Current Time: {} ms", current_millis);

        let utc = NaiveDateTime::from_timestamp_millis(current_millis).expect("Failed to convert time");
        let zoned_dt = found.from_utc_datetime(&utc);

        log::info!("Current Time naive: {}", zoned_dt.format("%Y-%m-%d %H:%M:%S"));
        let date_string = format!("{}", zoned_dt.format("%A %d %B %Y (%d/%m/%Y) WK %U"));
        let time_string = format!("{}", zoned_dt.format("%H%M%S"));

        if (time_string != current_time) {
            let def_scale : i32 = 5;
            let small_scale: i32 = 2;
            let offset_y : i32 = 10;

            for number in 0..time_string.len() {
                if let (Some(c1), Some(c2)) = (time_string.chars().nth(number), current_time.chars().nth(number)) {
                    if c1 != c2 {
                        let index: usize = c1.to_string().parse()?;

                        let is_small = number > 3;

                        let offset_x : i32 = if !is_small {
                            10 + (number as i32 * ((def_scale * 15) + 20))
                        }
                        else {
                            10 + (4 * ((def_scale * 15) + 20)) + ((number as i32- 4) * ((small_scale * 15) + 5))
                        };
                        let scale : i32 = if is_small { small_scale} else {def_scale};

                        let pixels = font_bmps[index].pixels();

                        for Pixel(position, color) in pixels {
                            let display_pixels = AlwaysSame { value: Rgb565::from(color) };
                            display.set_pixels(
                                (offset_x + position.x * scale) as u16,
                                (offset_y + position.y * scale) as u16,
                                (offset_x + (position.x + 1) * scale) as u16,
                                (offset_y + (position.y + 1) * scale) as u16,
                                display_pixels.into_iter().take((scale * (scale + 1)) as usize))
                                    .draw_context()?;
                        }
                    }
                }
            }

            current_time = time_string;
        }
        else if (date_string != current_date) {
            let cls_pixels = AlwaysSame {value: if (is_debug) { Rgb565::GREEN } else { Rgb565::BLACK } };
            display.set_pixels(10, 185, 450, 205, cls_pixels.into_iter().take(550*20) )
                .draw_context()?;

            draw_text(display, &date_string, Point::new(10, 200), Rgb565::WHITE)?;

            current_date = date_string;
        }
        else if ((current_millis - last_updated_at_a) > 50000) {
            let bmp_result = get_request_raw(&mut client, format!("{}/Time/Image/0", BASEURL));
            if let Ok(bmp_data) = bmp_result {
                let bmp = Bmp::<Rgb565>::from_slice(&bmp_data).unwrap();
                Image::new(&bmp, Point::new( 10, 220)).draw(display).draw_context()?;
            }
            else {
                client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?)
            }
            last_updated_at_a = current_millis;
        }
        else if ((current_millis - last_updated_at_b) > 50000) {
            let bmp_result = get_request_raw(&mut client, format!("{}/Time/Image/1", BASEURL));
            if let Ok(bmp_data) = bmp_result {
                let bmp = Bmp::<Rgb565>::from_slice(&bmp_data).unwrap();
                Image::new(&bmp, Point::new(250, 220)).draw(display).draw_context()?;
            }
            else {
                client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?)
            }
            last_updated_at_b = current_millis;
        }

        thread::sleep(Duration::from_millis(10));
    }
}