use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

pub fn render_text(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: String,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let surface = font
        .render(&text)
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let dims = texture.query();

    let target = Rect::new(x, y, dims.width, dims.height);
    canvas.copy(&texture, None, Some(target))?;

    canvas.present();

    Ok(())
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: String,
) {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);

    let lines = text.split("\r\n");

    let x = 0;
    let mut y = 10;
    for line in lines {
        render_text(canvas, texture_creator, font, line.to_string(), x, y);
        y += 32;
    }
}
