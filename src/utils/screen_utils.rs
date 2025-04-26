use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb565};
use crate::configuration::setup_display::DisplayDriver;
use crate::utils::always_same::AlwaysSame;
use crate::utils::simple_error::ContextExt;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;

pub fn clear_screen(mut display: &mut DisplayDriver, color: Rgb565) -> anyhow::Result<()> {
    let cls_pixels = AlwaysSame {value: color };
    display.set_pixels(0, 0, 500, 250, cls_pixels.into_iter().take(500*250) )
        .draw_context()?;

    let cls_pixels = AlwaysSame {value: color };
    display.set_pixels( 0,250,500,350, cls_pixels.into_iter().take(500 * 100) )
        .draw_context()?;

    Ok(())
}

pub fn draw_text(mut display: &mut DisplayDriver, text: &str, point: Point, color: Rgb565) -> anyhow::Result<()> {
    let loading_style = MonoTextStyle::new(&FONT_10X20, color);
    Text::new(text, point, loading_style)
        .draw(display)
        .draw_context()?;
    Ok(())
}