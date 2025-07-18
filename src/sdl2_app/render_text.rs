use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use super::constants::*;
use super::render_screen::get_screen_center;

pub fn draw_text(
    canvas: &mut WindowCanvas,
    text: &str,
    position: Rect, // Ta Rect definira območje (x, y, width, height) kjer naj bo besedilo centrirano
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    font_size: u16,
    text_color: Color,
    background: Option<Color>,
    custom_background :bool
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    
    // Ustvari površino s tekstom
    let font = ttf_context.load_font(PATH_TO_FONT, font_size)?;
    let text_surface = font
        .render(text)
        .blended(text_color)
        .map_err(|e| e.to_string())?;

    let (text_width, text_height) = (text_surface.width(), text_surface.height());


    let x = position.x() + (position.width() as i32 - text_width as i32) / 2;
    let y = position.y() + (position.height() as i32 - text_height as i32) / 2;
    let dest_rect = Rect::new(x, y, text_width, text_height);

    if let Some(bg_color) = background {
        canvas.set_draw_color(bg_color);
        if custom_background {
        // assert!(position.contains_rect(dest_rect), "when printing {text}");
        canvas.fill_rect(position)?; // Celoten podani pravokotnik
        } else {
            canvas.fill_rect(dest_rect)?;
        }
    }

    // if !position.contains_rect(dest_rect) {
    //     println!("premjahen rect za text narisat (draw_text)");
    // }

    let text_texture = texture_creator
        .create_texture_from_surface(&text_surface)
        .map_err(|e| e.to_string())?;
    
    canvas.copy(&text_texture, None, dest_rect)?;
    Ok(())
}

pub fn write_info(
    canvas: &mut WindowCanvas,
    string: &String,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    point_size: u16,
) -> Result<(), String> {
    let center = get_screen_center(&canvas);
    let pos = Point::new(center.x, center.y + SCREEN_CENTER_Y_OFFSET);
    let rect = Rect::from_center(pos, 0, 0);
    draw_text(
        canvas,
        &string,
        rect,
        &ttf_context,
        point_size,
        Color::RGB(0, 0, 0),
        Some(INFO_B_COLOR),
        false
    )?;
    Ok(())
}
