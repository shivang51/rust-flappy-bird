use raylib::{
    ffi::{GetRandomValue, Rectangle},
    misc::AsF32,
    prelude::{Color, RaylibDraw},
};

pub struct Pipe {
    x: i32,
    y: i32,
    height: i32,
}

impl Default for Pipe {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            height: 100,
        }
    }
}

const GAP: i32 = 100;
const MIN_HEIGHT: i32 = 100;
const WIDTH: i32 = 60;

impl Pipe {
    pub fn with_x(ctx: &mut raylib::prelude::RaylibHandle, x: i32) -> Self {
        let height: i32;
        unsafe {
            height = GetRandomValue(MIN_HEIGHT, ctx.get_screen_height() - MIN_HEIGHT - GAP) as i32;
        }
        Self { x, y: 0, height }
    }

    pub fn get_x(&mut self) -> i32 {
        self.x
    }

    pub fn with_x_dh(ctx: &mut raylib::prelude::RaylibDrawHandle, x: i32) -> Self {
        let height: i32;
        unsafe {
            height = GetRandomValue(MIN_HEIGHT, ctx.get_screen_height() - MIN_HEIGHT - GAP) as i32;
        }
        Self { x, y: 0, height }
    }

    pub fn draw(&mut self, ctx: &mut raylib::prelude::RaylibDrawHandle) {
        ctx.draw_rectangle(self.x, 0, WIDTH, self.height, Color::GREEN);
        ctx.draw_rectangle(
            self.x,
            self.height + GAP,
            WIDTH,
            ctx.get_screen_height() - self.height - GAP,
            Color::GREEN,
        );

        self.x -= 2;
    }

    pub fn get_recs(&mut self, ctx: &mut raylib::prelude::RaylibDrawHandle) -> [Rectangle; 2] {
        let rec_top = Rectangle {
            x: self.x.as_f32(),
            y: self.y.as_f32(),
            width: WIDTH.as_f32(),
            height: self.height.as_f32(),
        };
        let rec_bottom = Rectangle {
            x: self.x.as_f32(),
            y: (self.height + GAP).as_f32(),
            width: WIDTH.as_f32(),
            height: (ctx.get_screen_height() - self.height - GAP).as_f32(),
        };
        [rec_top, rec_bottom]
    }
}
