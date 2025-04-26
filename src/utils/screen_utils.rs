use embedded_graphics::{Drawable, Pixel};
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb565, Rgb888};
use crate::configuration::setup_display::DisplayDriver;
use crate::utils::always_same::AlwaysSame;
use crate::utils::simple_error::ContextExt;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;
use tinybmp::Pixels;

pub fn clear_screen(mut display: &mut DisplayDriver, color: Rgb565) -> anyhow::Result<()> {
    //We have to do this in batches otherwise bus is overwhelmed
    draw_box(display, color, 0,0,500,250)?;
    draw_box(display, color, 0,250,500,350)?;

    Ok(())
}

pub fn draw_text(mut display: &mut DisplayDriver, text: &str, point: Point, color: Rgb565) -> anyhow::Result<()> {
    let loading_style = MonoTextStyle::new(&FONT_10X20, color);
    Text::new(text, point, loading_style)
        .draw(display)
        .draw_context()?;
    Ok(())
}

pub fn draw_pixels_at_scale(mut display: &mut DisplayDriver, pixels: Pixels<Rgb888>,  point: Point, scale: i32) -> anyhow::Result<()> {
    for Pixel(position, color) in pixels {
        draw_box(
            display,
            Rgb565::from(color),
            (point.x + position.x * scale) as u16,
            (point.y + position.y * scale) as u16,
            (point.x + (position.x + 1) * scale) as u16,
            (point.y + (position.y + 1) * scale) as u16)?
    }

    Ok(())
}

pub fn draw_box(mut display: &mut DisplayDriver, color: Rgb565, start_x: u16, start_y: u16, end_x: u16, end_y: u16) -> anyhow::Result<()> {
    //On the specific model of CYD I have, embedded_graphics refuses to draw on the edges of the screen, fix by using lower level
    //set_pixels commands

    let pixels = AlwaysSame { value: color };
    let x_diff = (end_x as u32 - start_x as u32);
    let y_diff = (end_y as u32 - start_y as u32);

    if (x_diff < 0 || y_diff < 0) {
        anyhow::bail!("draw_box invalid coords");
    }

    display.set_pixels(start_x, start_y, end_x, end_y, pixels.into_iter().take(((x_diff+1) * (y_diff+1)) as usize))
        .draw_context()?;

    Ok(())
}