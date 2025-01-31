use macroquad::prelude::*;

use crate::{utils::{cells::Cells, directions::Directions, grid::Grid, movercell::MoverCell}, widgets::fpsviewer::FpsViewer};

pub struct GameScreen {
    grid: Grid,
    pan_velocity: (f32, f32),
    pan_speed: f32,
    fps_viewer: FpsViewer,
    place_direction: Directions,
}

impl GameScreen {
    pub async fn new(w: u32, h: u32) -> GameScreen {
        GameScreen {
            grid: Grid::new(w, h).await,
            pan_velocity: (0.0, 0.0),
            pan_speed: 7.5,
            fps_viewer: FpsViewer::new(20.0, 20.0, 60),
            place_direction: Directions::Right,
        }
    }

    pub async fn update(&mut self, tick: u32) {
        self.fps_viewer.update();

        if is_key_down(KeyCode::D) {
            self.pan_velocity.0 = -self.pan_speed;
        }
        if is_key_down(KeyCode::A) {
            self.pan_velocity.0 = self.pan_speed;
        }
        if is_key_down(KeyCode::S) {
            self.pan_velocity.1 = -self.pan_speed;
        }
        if is_key_down(KeyCode::W) {
            self.pan_velocity.1 = self.pan_speed;
        }

        if is_key_pressed(KeyCode::Q) {
            self.place_direction = match self.place_direction {
                Directions::Up => Directions::Right,
                Directions::Right => Directions::Down,
                Directions::Down => Directions::Left,
                Directions::Left => Directions::Up,
            };
        }

        if is_key_pressed(KeyCode::E) {
            self.place_direction = match self.place_direction {
                Directions::Up => Directions::Left,
                Directions::Left => Directions::Down,
                Directions::Down => Directions::Right,
                Directions::Right => Directions::Up,
            };
        }
        
        self.grid.set_draw_offset(
            self.grid.draw_offset.0 + self.pan_velocity.0 as i32,
            self.grid.draw_offset.1 + self.pan_velocity.1 as i32,
        );

        self.pan_velocity.0 *= 0.9;
        self.pan_velocity.1 *= 0.9;

        if is_key_pressed(KeyCode::Space) {
            self.grid.is_paused = !self.grid.is_paused;
        }

        let mouse_pos = mouse_position();
        let coords_x = (((mouse_pos.0 - self.grid.draw_offset.0 as f32) / 64.0) as i32).max(0);
        let coords_y = (((mouse_pos.1 - self.grid.draw_offset.1 as f32) / 64.0) as i32).max(0);
        let dest_x = coords_x as f32 * 64.0 + self.grid.draw_offset.0 as f32;
        let dest_y = coords_y as f32 * 64.0 + self.grid.draw_offset.1 as f32;
        if is_mouse_button_down(MouseButton::Left) {
            self.grid.set_cell(coords_x as u32, coords_y as u32, Cells::MoverCell(MoverCell::new(dest_x, dest_y, Some(self.place_direction.clone())).await));
        }

        self.grid.update(tick).await;
    }

    pub fn draw(&self, tick: u32) {
        clear_background(BLACK);
        self.grid.draw();
        self.fps_viewer.draw();
        draw_text(
            &format!("Paused: {}, Tick: {}, Is Divisable by 10: {}", self.grid.is_paused, tick, tick % 10 == 0),
            20.0,
            128.0,
            32.0,
            WHITE,
        );
    }
}
