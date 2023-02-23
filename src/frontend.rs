use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use crate::config::FONT_SIZE;

pub struct Frontend<'a> {
    canvas: &'a mut WindowCanvas,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &'a sdl2::ttf::Font<'a, 'a>,
    x: i32,
    y: i32,
}

impl Frontend<'_> {
    pub fn new<'a>(
        canvas: &'a mut WindowCanvas,
        texture_creator: &'a TextureCreator<WindowContext>,
        font: &'a sdl2::ttf::Font<'a, 'a>,
        x: i32,
        y: i32,
    ) -> Frontend<'a> {
        Frontend {
            canvas,
            texture_creator,
            font,
            x,
            y,
        }
    }

    pub fn render_text(&mut self, text: String) -> Result<(), String> {
        let surface = self
            .font
            .render(&text)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let dims = texture.query();

        let target = Rect::new(self.x, self.y, dims.width, dims.height);
        self.canvas.copy(&texture, None, Some(target))?;

        self.canvas.present();

        Ok(())
    }

    pub fn render(&mut self, text: String) {
        let color = Color::RGB(0, 0, 0);
        self.canvas.set_draw_color(color);

        let lines = text.split("\n");

        let mut y = 10;
        for line in lines {
            self.render_text(line.to_string());
            self.y += FONT_SIZE as i32;
        }
    }
}
