use macroquad::prelude::*;

pub struct FpsViewer {
    pub position: (f32, f32),
    pub fps_history: [i32; 60],
    pub max_fps: i32,
}

impl FpsViewer {
    pub fn new(x: f32, y: f32, max: i32) -> FpsViewer {
        FpsViewer {
            position: (x, y),
            fps_history: [0; 60],
            max_fps: max,
        }
    }

    pub fn update(&mut self) {
        let fps = get_fps();
        for i in 0..59 {
            self.fps_history[i] = self.fps_history[i + 1];
        }
        self.fps_history[59] = fps;
    }

    pub fn draw(&self) {
        const WIDTH: f32 = 256.0;
        const HEIGHT: f32 = 64.0;
        const BAR_WIDTH: f32 = WIDTH / 60.0;

        for i in 0..60 {
            let color = Color::new(0.0, 1.0, 0.0, 0.5);

            draw_rectangle(
                self.position.0 + i as f32 * BAR_WIDTH,
                self.position.1 + HEIGHT,
                BAR_WIDTH,
                -self.fps_history[i] as f32 / self.max_fps as f32 * HEIGHT,
                color,
            );
        }

        draw_text(
            &format!("FPS: {}", self.fps_history[59]),
            self.position.0,
            self.position.1 + HEIGHT + 20.0,
            32.0,
            WHITE,
        );
    }
}
