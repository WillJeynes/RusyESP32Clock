use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use esp_idf_svc::http::client::EspHttpConnection;
use crate::configuration::setup_display::DisplayDriver;
use crate::configuration::setup_wifi::get_request_raw;
use embedded_svc::http::client::Client;
use tinybmp::Bmp;
use crate::utils::simple_error::ContextExt;

pub fn draw_image(mut display: &mut DisplayDriver, client: &mut Client<EspHttpConnection>, url: String, point: Point) -> anyhow::Result<()> {
    let bmp_result = get_request_raw(client, url);
    if let Ok(bmp_data) = bmp_result {
        let bmp = Bmp::<Rgb565>::from_slice(&bmp_data).map_err(|_| anyhow::anyhow!("BMP parse failed"))?;
        Image::new(&bmp, point).draw(display).draw_context()?;
    }
    else {
        anyhow::bail!("Failed to get bitmap");
    }

    Ok(())
}