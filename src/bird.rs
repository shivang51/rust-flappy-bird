use raylib::{ffi::Rectangle, misc::AsF32, prelude::*};

pub struct Bird {
    x: i32,
    y: i32,
}

impl Default for Bird {
    fn default() -> Self {
        Bird { x: 0, y: 0 }
    }
}

const SIZE: i32 = 40;
const HEAD_SIZE: i32 = 20;
const DY: i32 = 3;

impl Bird {
    pub fn new(ctx: &mut raylib::prelude::RaylibHandle) -> Self {
        let x = (ctx.get_screen_width().as_f32() * 0.25) as i32;
        let y = (ctx.get_screen_height().as_f32() * 0.5) as i32;
        Bird { x, y }
    }

    pub fn reset(&mut self, ctx: &mut raylib::prelude::RaylibDrawHandle) {
        let x = (ctx.get_screen_width().as_f32() * 0.25) as i32;
        let y = (ctx.get_screen_height().as_f32() * 0.5) as i32;
        self.x = x;
        self.y = y;
    }

    pub fn draw(&mut self, ctx: &mut raylib::prelude::RaylibDrawHandle) {
        ctx.draw_rectangle(self.x, self.y, SIZE, SIZE, Color::BLUE);
        ctx.draw_triangle(
            Vector2 {
                x: (self.x + SIZE).as_f32(),
                y: self.y.as_f32(),
            },
            Vector2 {
                x: (self.x + SIZE).as_f32(),
                y: (self.y + SIZE).as_f32(),
            },
            Vector2 {
                x: (self.x + SIZE + HEAD_SIZE).as_f32(),
                y: (self.y + HEAD_SIZE).as_f32(),
            },
            Color::BLUE,
        )
    }

    pub fn update(&mut self, ctx: &mut raylib::prelude::RaylibDrawHandle) {
        let key_down = ctx.is_key_down(KeyboardKey::KEY_UP);

        if key_down && self.y > 0 {
            self.y -= DY;
        } else if !key_down && self.y + SIZE < ctx.get_screen_height() {
            self.y += DY;
        }
    }

    pub fn get_rec(&mut self) -> Rectangle {
        Rectangle {
            x: self.x.as_f32(),
            y: self.y.as_f32(),
            width: SIZE.as_f32(),
            height: SIZE.as_f32(),
        }
    }
}
