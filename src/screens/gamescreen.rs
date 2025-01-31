use macroquad::prelude::*;

use crate::{utils::{cells::Cells, directions::Directions, generatorcell::GeneratorCell, grid::Grid, helper::degrees2radians, movercell::MoverCell, pushcell::PushCell}, widgets::fpsviewer::FpsViewer};

pub struct GameScreen {
    grid: Grid,
    pan_velocity: (f32, f32),
    pan_speed: f32,
    fps_viewer: FpsViewer,
    place_direction: Directions,
    selected_cell: Cells,
}

impl GameScreen {
    pub async fn new(w: u32, h: u32) -> GameScreen {
        GameScreen {
            grid: Grid::new(w, h).await,
            pan_velocity: (0.0, 0.0),
            pan_speed: 7.5,
            fps_viewer: FpsViewer::new(20.0, 20.0, 60),
            place_direction: Directions::Right,
            selected_cell: Cells::MoverCell(MoverCell::new(0.0, 0.0, Some(Directions::Right)).await),
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

        if is_key_pressed(KeyCode::E) {
            self.place_direction = match self.place_direction {
                Directions::Up => Directions::Right,
                Directions::Right => Directions::Down,
                Directions::Down => Directions::Left,
                Directions::Left => Directions::Up,
            };
        }

        if is_key_pressed(KeyCode::Q) {
            self.place_direction = match self.place_direction {
                Directions::Up => Directions::Left,
                Directions::Left => Directions::Down,
                Directions::Down => Directions::Right,
                Directions::Right => Directions::Up,
            };
        }

        if is_key_pressed(KeyCode::Z) {
            self.selected_cell = match self.selected_cell {
                Cells::MoverCell(_) => Cells::GeneratorCell(GeneratorCell::new(0.0, 0.0, Some(self.place_direction.clone())).await),
                Cells::PushCell(_) => Cells::MoverCell(MoverCell::new(0.0, 0.0, Some(self.place_direction.clone())).await),
                Cells::GeneratorCell(_) => Cells::PushCell(PushCell::new(0.0, 0.0, None).await),
                _ => Cells::MoverCell(MoverCell::new(0.0, 0.0, Some(self.place_direction.clone())).await),
            };
        }

        if is_key_pressed(KeyCode::X) {
            self.selected_cell = match self.selected_cell {
                Cells::MoverCell(_) => Cells::PushCell(PushCell::new(0.0, 0.0, None).await),
                Cells::PushCell(_) => Cells::GeneratorCell(GeneratorCell::new(0.0, 0.0, Some(self.place_direction.clone())).await),
                Cells::GeneratorCell(_) => Cells::MoverCell(MoverCell::new(0.0, 0.0, Some(self.place_direction.clone())).await),
                _ => Cells::MoverCell(MoverCell::new(0.0, 0.0, Some(self.place_direction.clone())).await),
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
            // self.grid.set_cell(coords_x as u32, coords_y as u32, Cells::MoverCell(MoverCell::new(dest_x, dest_y, Some(self.place_direction.clone())).await));
            let moved_selected_cell = match self.selected_cell {
                Cells::MoverCell(ref cell) => Cells::MoverCell(MoverCell::new(dest_x, dest_y, Some(self.place_direction.clone())).await),
                Cells::PushCell(ref cell) => Cells::PushCell(PushCell::new(dest_x, dest_y, None).await),
                Cells::GeneratorCell(ref cell) => Cells::GeneratorCell(GeneratorCell::new(dest_x, dest_y, Some(self.place_direction.clone())).await),
                _ => Cells::MoverCell(MoverCell::new(dest_x, dest_y, Some(self.place_direction.clone())).await),
            };
            self.grid.set_cell(coords_x as u32, coords_y as u32, moved_selected_cell);
        }

        self.grid.update(tick).await;
    }

    pub async fn draw(&self, tick: u32) {
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
        
        // Draw the selected cell
        let mouse_pos = mouse_position();
        let coords_x = (((mouse_pos.0 - self.grid.draw_offset.0 as f32) / 64.0) as i32).max(0);
        let coords_y = (((mouse_pos.1 - self.grid.draw_offset.1 as f32) / 64.0) as i32).max(0);
        let dest_x = coords_x as f32 * 64.0 + self.grid.draw_offset.0 as f32;
        let dest_y = coords_y as f32 * 64.0 + self.grid.draw_offset.1 as f32;
        let texture = match self.selected_cell {
            Cells::MoverCell(ref cell) => &cell.texture,
            Cells::PushCell(ref cell) => &cell.texture,
            Cells::GeneratorCell(ref cell) => &cell.texture,
            _ => &MoverCell::new(0.0, 0.0, Some(Directions::Right)).await.texture,
        };
        draw_texture_ex(
            texture,
            dest_x,
            dest_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                rotation: match self.place_direction {
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
