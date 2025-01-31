use macroquad::prelude::*;

pub struct TitleScreen {
    selected: i32,
    options: Vec<String>,
    options_translations: Vec<String>,
}

impl TitleScreen {
    pub fn new() -> TitleScreen {
        TitleScreen {
            selected: 0,
            options: vec!["new".to_string(), "load".to_string(), "exit".to_string()],
            options_translations: vec!["New Game".to_string(), "Load Game".to_string(), "Exit".to_string()],
        }
    }

    pub fn update(&mut self) -> Option<String> {
        if is_key_pressed(KeyCode::Down) {
            self.selected += 1;
            if self.selected >= self.options.len() as i32 {
                self.selected = 0;
            }
        }
        if is_key_pressed(KeyCode::Up) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.options.len() as i32 - 1;
            }
        }
        if is_key_pressed(KeyCode::Enter) {
            return Some(self.options[self.selected as usize].clone());
        }

        None
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        draw_text("cell_machine_mystic_mod.rs", 128.0, 128.0, 64.0, YELLOW);
        draw_text("Alpha v0.1.4 Suuuuper Buggy!", 128.0, 192.0, 32.0, GRAY);
        for (i, _option) in self.options.iter().enumerate() {
            let color = if i as i32 == self.selected { WHITE } else { GRAY };
            let text = if i as i32 == self.selected { format!("> {}", self.options_translations[i]) } else { self.options_translations[i].clone() };
            draw_text(text.as_str(), 128.0, 256.0 + 40.0 * i as f32, 32.0, color);
        }
    }
}
