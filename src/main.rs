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
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::TZ_VARIANTS;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb565;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use crate::configuration::setup_display::setup_display;
use crate::configuration::setup_wifi::{connect_wifi, get_request, get_request_raw};
use embedded_svc::{
    http::{client::Client as HttpClient},
};
use embedded_svc::http::status::INFO;
use crate::utils::always_same::AlwaysSame;
use esp_idf_hal::delay::Ets;
use esp_idf_sys::{esp_task_wdt_reset, esp_timer_get_time};
use tinybmp::Bmp;

const SSID: &str = std::env!("SSID");
const PASSWORD: &str = std::env!("PASSWORD");

const BASEURL: &str = std::env!("BASEURL");

fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let mut delay = Ets;

    //Init Logging
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");

    //Init Peripherals
    let peripherals = Peripherals::take().unwrap();

    //Display
    let mut display = setup_display(peripherals.pins, peripherals.spi2)?;

    let blue_pixels = AlwaysSame {value: Rgb565::BLUE};
    display.set_pixels( 0,0,500,250, blue_pixels.into_iter().take(500*250) )
        .map_err(|_| Box::<dyn Error>::from("draw world"))?;

    let red_pixels = AlwaysSame {value: Rgb565::RED};
    display.set_pixels( 0,250,500,350, red_pixels.into_iter().take(500 * 100) )
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
    let request_string = get_request(&mut client, format!("{}/Time/GetCurrentTime", BASEURL))?;
    let request_clock = request_string.parse::<i64>().unwrap();
    let request_millis = unsafe {esp_timer_get_time()} / 1000;
    log::info!("Got Time Request {} at {}",request_string,  request_millis);

    display.clear(Rgb565::BLUE).map_err(|_| Box::<dyn Error>::from("draw world"))?;

    let mut current_date = String::from("");
    let mut current_time = String::from("");

    let mut last_updated_at_a = 0;
    let mut last_updated_at_b = 0;

    //Time-zoning
    let location_name = "Europe/London";

    let found = TZ_VARIANTS.iter().find(|v| {
        v.name() == location_name
    }).unwrap();

    //Temp, add env based font loading
    let fontBitmaps = [
        include_bytes!("Fonts/Default/1.bmp"),
        include_bytes!("Fonts/Default/2.bmp"),
        include_bytes!("Fonts/Default/3.bmp"),
        include_bytes!("Fonts/Default/4.bmp"),
        include_bytes!("Fonts/Default/5.bmp"),
        include_bytes!("Fonts/Default/6.bmp"),
        include_bytes!("Fonts/Default/7.bmp"),
        include_bytes!("Fonts/Default/8.bmp"),
        include_bytes!("Fonts/Default/9.bmp"),
    ];

    let as_bmps = fontBitmaps.map(|data|  Bmp::<Rgb565>::from_slice(data).unwrap());

    

    loop {
        let current_millis = unsafe {esp_timer_get_time()} / 1000;
        let difference = current_millis - request_millis;
        log::info!("Current Time difference: {} ms", difference);
        let current_millis = request_clock + difference;
        log::info!("Current Time: {} ms", current_millis);

        //This is depricated but chrono tz unhappy otherwise
        let mut naive = NaiveDateTime::from_timestamp_millis(current_millis).expect("Failed to convert time");
        let zoned = found.from_utc_datetime(&naive);


        log::info!("Current Time naive: {}", zoned.format("%Y-%m-%d %H:%M:%S"));
        let date_string = format!("{}", zoned.format("%A %d %B %Y (%d/%m/%Y) WK %U"));
        let time_string = format!("{}", zoned.format("%H%M%S"));
        //TODO: Timezones

        if (time_string != current_time) {
            //TODO: replace with large display
            let white_pixels = AlwaysSame {value: Rgb565::WHITE};
            display.set_pixels( 10,80,300,100, white_pixels.into_iter().take(490*20) )
                .map_err(|_| Box::<dyn Error>::from("draw world"))?;

            let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
            Text::new(&time_string, Point::new(10, 100), text_style)
                .draw(&mut display)
                .map_err(|_| Box::<dyn Error>::from("draw world"))?;

            current_time = time_string;
        }
        else if (date_string != current_date) {
            let white_pixels = AlwaysSame {value: Rgb565::WHITE};
            display.set_pixels( 10,185,400,205, white_pixels.into_iter().take(500*20) )
                .map_err(|_| Box::<dyn Error>::from("draw world"))?;

            let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
            Text::new(&date_string, Point::new(10, 200), text_style)
                .draw(&mut display)
                .map_err(|_| Box::<dyn Error>::from("draw world"))?;

            current_date = date_string;
        }
        else if ((current_millis - last_updated_at_a) > 5000) {
            let bmp_data = get_request_raw(&mut client, format!("{}/Time/Image/0", BASEURL)).map_err(|_| Box::<dyn Error>::from("draw world"))?;
            let bmp = Bmp::<Rgb565>::from_slice(&bmp_data).unwrap();
            Image::new(&bmp, Point::new( 10, 220)).draw(&mut display).map_err(|_| Box::<dyn Error>::from("draw world"))?;

            last_updated_at_a = current_millis;
        }
        else if ((current_millis - last_updated_at_b) > 5000) {
            let bmp_data = get_request_raw(&mut client, format!("{}/Time/Image/1", BASEURL)).map_err(|_| Box::<dyn Error>::from("draw world"))?;
            let bmp = Bmp::<Rgb565>::from_slice(&bmp_data).unwrap();
            Image::new(&bmp, Point::new( 250, 220)).draw(&mut display).map_err(|_| Box::<dyn Error>::from("draw world"))?;

            last_updated_at_b = current_millis;
        }

        thread::sleep(Duration::from_millis(5));
    }
}