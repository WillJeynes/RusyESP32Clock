use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
    geometry::Point,
};
use esp_idf_svc::hal::{gpio, prelude::Peripherals};
use esp_idf_hal::{
    delay::Ets,
    spi::{config::{Config, DriverConfig}, Dma, SpiDeviceDriver},
    units::MegaHertz,
};
use mipidsi::Builder;
use std::error::Error;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    io::Write,
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};

const SSID: &str = "VM4597057";
const PASSWORD: &str = "hm5dcBcfSnvx";
fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Reset, -1 or 4
    let rst = gpio::PinDriver::output(pins.gpio4)?;
    // Data Command control pin
    let dc = gpio::PinDriver::output(pins.gpio2)?;

    // Espressif built-in delay provider for small delays
    let mut delay = Ets;

    // Pin 14, Serial Clock
    let sclk = pins.gpio14;
    let spi = peripherals.spi2;
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
    let mut display = Builder::ili9341_rgb565(di)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .with_orientation(mipidsi::options::Orientation::Landscape(false))  // Mirror on text
        //.with_display_size(320,240)
        .init(&mut delay, Some(rst))
        .map_err(|_| Box::<dyn Error>::from("display init"))?;


    // Pin 27, Backlight
    let mut bl = gpio::PinDriver::output(pins.gpio27)?;
    // Turn on backlight
    bl.set_high()?;

    // Force the GPIO to hold its high state
    core::mem::forget(bl);

    // Clear the screen with black
    display.clear(Rgb565::BLACK)
        .map_err(|_| Box::<dyn Error>::from("clear display"))?;

    // Create text style for "Hello"
    let style_hello = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("TS", Point::new(5, 10), style_hello)
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw hello"))?;

    // Change text color to blue for "World"
    let style_world = MonoTextStyle::new(&FONT_10X20, Rgb565::BLUE);
    Text::new("PMO !!!!!!!", Point::new(160, 26), style_world)
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw world"))?;

    // Initialize logging
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");


    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;


    let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);

    // GET
    let e = get_request(&mut client)?;


    let style_world = MonoTextStyle::new(&FONT_10X20, Rgb565::BLUE);
    Text::new(&e, Point::new(160, 126), style_world)
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw world"))?;

    display.set_pixels( 50,300,500,500,
                        [Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE ,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE ,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE ,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE ,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE ,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE ,Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE, Rgb565::WHITE,Rgb565::WHITE])
        .map_err(|_| Box::<dyn Error>::from("draw world"))?;

    Ok(())
}


fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    log::info!("Wifi started");

    wifi.connect()?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up");

    Ok(())
}

fn get_request(client: &mut HttpClient<EspHttpConnection>) -> anyhow::Result<String> {
    // Prepare headers and URL
    let headers = [("accept", "text/plain")];
    let url = "http://http.badssl.com/";

    // Send request
    //
    // Note: If you don't want to pass in any headers, you can also use `client.get(url, headers)`.
    let request = client.request(Method::Get, url, &headers)?;
    log::info!("-> GET {}", url);
    let mut response = request.submit()?;

    // Process response
    let status = response.status();
    log::info!("<- {}", status);
    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    log::info!("Read {} bytes", bytes_read);
    match std::str::from_utf8(&buf[0..bytes_read]) {
        Ok(body_string) => {
            return Ok(body_string.to_owned());
        },
        Err(e) => log::error!("Error decoding response body: {}", e),
    };

    return Err(anyhow::anyhow!("Error decoding response body"));
}