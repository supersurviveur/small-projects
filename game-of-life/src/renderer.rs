use macroquad::{
    color::{BLACK, RED, WHITE},
    input::is_key_down,
    texture::{Image, Texture2D, draw_texture},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use crate::GameOfLife;

pub struct Renderer {
    scale: f32,
    center_x: u32,
    center_y: u32,
    image: Image,
    texture: Texture2D,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new(1., 0, 0)
    }
}

impl Renderer {
    pub fn new(scale: f32, center_x: u32, center_y: u32) -> Self {
        let image = Image::gen_image_color(screen_width() as u16, screen_height() as u16, BLACK);
        Self {
            scale,
            center_x,
            center_y,
            texture: Texture2D::from_image(&image),
            image,
        }
    }
    pub fn create_frame(&mut self, game: &GameOfLife) {
        let w = screen_width() as u16;
        let h = screen_height() as u16;

        // Resize the image and the texture if the screen size have changed
        if w != self.image.width || h != self.image.height {
            self.image.bytes.fill(0);
            self.image.bytes.resize(w as usize * h as usize * 4, 0);
            self.image.height = h;
            self.image.width = w;
            self.texture = Texture2D::from_image(&self.image);
        }

        for x in 0..w {
            let game_x = x as i32 - w as i32 / 2;
            let game_x = (game_x as f32 * self.scale).round() as i32;
            let game_x = self.center_x as i32 + game_x + game.width as i32 / 2;
            if game_x < 0 || game_x as usize >= game.width {
                for y in 0..h {
                    self.image.set_pixel(x as u32, y as u32, BLACK);
                }
                continue;
            }

            for y in 0..h {
                let game_y = y as i32 - h as i32 / 2;
                let game_y = (game_y as f32 * self.scale).round() as i32;
                let game_y = self.center_y as i32 + game_y + game.height as i32 / 2;
                if game_y < 0 || game_y as usize >= game.height {
                    self.image.set_pixel(x as u32, y as u32, BLACK);
                    continue;
                }

                self.image.set_pixel(
                    x as u32,
                    y as u32,
                    if game.grid[(game_x as usize, game_y as usize)] {
                        WHITE
                    } else {
                        BLACK
                    },
                );
            }
        }
    }
    pub fn events(&mut self) {
        if is_key_down(macroquad::input::KeyCode::Right) {
            self.center_x += 10;
        }
        if is_key_down(macroquad::input::KeyCode::Left) {
            self.center_x -= 10;
        }
        if is_key_down(macroquad::input::KeyCode::Down) {
            self.center_y += 10;
        }
        if is_key_down(macroquad::input::KeyCode::Up) {
            self.center_y -= 10;
        }
        if is_key_down(macroquad::input::KeyCode::Z) {
            self.scale *= 0.85;
        }
        if is_key_down(macroquad::input::KeyCode::S) {
            self.scale /= 0.85;
        }
    }
    pub async fn render_frame(&mut self) {
        self.texture.update(&self.image);
        draw_texture(&self.texture, 0., 0., WHITE);
        next_frame().await
    }
    pub async fn render(&mut self, game: &GameOfLife) {
        self.events();
        self.create_frame(game);
        self.render_frame().await;
    }
}
