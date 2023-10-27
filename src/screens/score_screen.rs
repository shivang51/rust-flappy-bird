use raylib::{misc::AsF32, prelude::*};

pub struct ScoreScreen {}

impl ScoreScreen {
    pub fn draw(ctx: &mut RaylibDrawHandle, score: String) {
        let your_score = String::from("Your Score ") + &score;

        let mut x = (ctx.get_screen_width().as_f32() * 0.5) as i32;
        let y = (ctx.get_screen_height().as_f32() * 0.5) as i32;
        let size = measure_text(your_score.as_str(), 64);
        x -= size / 2;
        ctx.draw_text(your_score.as_str(), x, y, 64, Color::WHITE);
    }
}
