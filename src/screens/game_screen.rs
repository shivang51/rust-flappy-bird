use raylib::{ffi::CheckCollisionRecs, prelude::*};

use crate::{bird::Bird, pipes::Pipes};

pub struct GameScreen {
    pipes: Pipes,
    bird: Bird,
    score: i32,
    game_over: bool,
}

impl GameScreen {
    pub fn init(hwnd: &mut RaylibHandle) -> Self {
        Self {
            pipes: Pipes::init(hwnd),
            bird: Bird::new(hwnd),
            score: 0,
            game_over: false,
        }
    }

    pub fn reset(&mut self, ctx: &mut RaylibDrawHandle) {
        self.pipes.reset(ctx);
        self.bird.reset(ctx);
        self.score = 0;
        self.game_over = false;
    }

    pub fn draw(&mut self, ctx: &mut RaylibDrawHandle) {
        self.pipes.update(ctx);
        self.bird.update(ctx);

        self.bird.draw(ctx);
        self.pipes.draw(ctx);

        ctx.draw_text(self.score.to_string().as_str(), 50, 50, 32, Color::WHITE);

        unsafe {
            let bird_rec = self.bird.get_rec();
            let [pipe_top, pipe_bottom] = self.pipes[0].get_recs(ctx);

            let pipe_corner_r = pipe_top.x + pipe_top.width;

            if CheckCollisionRecs(pipe_top, bird_rec) || CheckCollisionRecs(pipe_bottom, bird_rec) {
                self.game_over = true;
            } else if bird_rec.x == pipe_corner_r {
                self.score += 5;
            }
        }
    }

    pub fn is_game_over(&mut self) -> bool {
        self.game_over
    }

    pub fn get_score(&mut self) -> i32 {
        self.score
    }
}
