use macroquad::prelude::*;

use super::{directions::Directions, helper::degrees2radians};

#[derive(Clone)]
pub struct GeneratorCell {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub direction: Directions,
}

impl GeneratorCell {
    pub async fn new(x: f32, y: f32, dir: Option<Directions>) -> GeneratorCell {
        let texture = load_texture("assets/generatorcell.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        GeneratorCell {
            x,
            y,
            texture,
            direction: dir.unwrap_or(Directions::Right),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                rotation: match self.direction {
                    Directions::Up => degrees2radians(270.0),
                    Directions::Down => degrees2radians(90.0),
                    Directions::Left => degrees2radians(180.0),
                    Directions::Right => degrees2radians(0.0),
                },
                ..Default::default()
            },
        );
    }
}
