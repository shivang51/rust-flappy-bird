mod bird;
mod pipe;
mod pipes;
mod screens;

use raylib::prelude::*;
use screens::{game_screen::GameScreen, score_screen::ScoreScreen, welcome_screen::WelcomeScreen};

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 500).title("Flappy Bird").build();

    rl.set_target_fps(60);

    let mut game_screen = GameScreen::init(&mut rl);

    let mut screen_ind = 0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        match screen_ind {
            0 => {
                WelcomeScreen::draw(&mut d);
                if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    screen_ind = 1;
                }
            }
            1 => {
                game_screen.draw(&mut d);
                if game_screen.is_game_over() {
                    screen_ind = 2;
                }
            }
            2 => {
                ScoreScreen::draw(&mut d, game_screen.get_score().to_string());
                if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    game_screen.reset(&mut d);
                    screen_ind = 1;
                }
            }
            _ => {}
        }
    }
}
