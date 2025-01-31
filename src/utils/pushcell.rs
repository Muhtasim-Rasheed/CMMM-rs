use macroquad::prelude::*;

#[derive(Clone)]
pub struct PushCell {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
}

impl PushCell {
    pub async fn new(x: f32, y: f32) -> PushCell {
        let texture = load_texture("assets/pushcell.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        PushCell {
            x,
            y,
            texture,
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
                ..Default::default()
            },
        );
    }
}
