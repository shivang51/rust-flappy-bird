use raylib::{
    ffi::{GetFontDefault, MeasureTextEx},
    misc::AsF32,
    prelude::*,
};

pub struct WelcomeScreen {}

impl WelcomeScreen {
    pub fn draw(ctx: &mut RaylibDrawHandle) {
        let mut x = (ctx.get_screen_width().as_f32() * 0.5) as i32;
        let mut y = (ctx.get_screen_height().as_f32() * 0.5) as i32;
        unsafe {
            let font = GetFontDefault();
            let size = MeasureTextEx(
                font,
                "Flappy Bird".as_ptr() as *const i8,
                64.as_f32(),
                font.charsPadding.as_f32(),
            );

            print!("");

            x -= (size.x as i32) / 2;
            y -= (size.y as i32) / 2;
        }
        ctx.draw_text("Flappy Bird", x, y, 64, Color::WHITE);
    }
}
