use macroquad::prelude::*;
use screens::{gamescreen::GameScreen, titlescreen::TitleScreen};

mod screens;
mod utils;
mod widgets;

fn window_conf() -> Conf {
    Conf {
        window_title: "CellMachineMysticMod.rs".to_string(),
        window_width: 1600,
        window_height: 900,
        ..Default::default()
    }
}

async fn game_loop() {
    let mut game_screen = GameScreen::new(50, 50).await;

    let mut tick = 0;
    loop {
        game_screen.update(tick).await;
        game_screen.draw(tick);

        tick += 1;
        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut title_screen = TitleScreen::new();
    let mut selected_option;

    loop {
        selected_option = title_screen.update();
        title_screen.draw();

        if let Some(_option) = &selected_option {
            break;
        }

        next_frame().await
    }

    match selected_option.unwrap().as_str() {
        "new" => {
            game_loop().await;
        }
        "load" => {
            println!("Load Game");
        }
        "exit" => {
            println!("Exit");
        }
        _ => {
            println!("Unknown Option");
        }
    }
}
