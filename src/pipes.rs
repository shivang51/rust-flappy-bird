use std::collections::VecDeque;

use raylib::{misc::AsF32, prelude::RaylibDrawHandle, RaylibHandle};

use crate::pipe::Pipe;

use std::ops::{Index, IndexMut};

pub struct Pipes {
    pipes: VecDeque<Pipe>,
}

const PIPES_DISTANCE: i32 = 350;

impl Pipes {
    pub fn init(ctx: &mut RaylibHandle) -> Pipes {
        let mut pipes: VecDeque<Pipe> = VecDeque::new();
        let mid = (ctx.get_screen_width().as_f32() * 0.5) as i32;
        for i in 0..5 {
            let pipe = Pipe::with_x(ctx, mid + (PIPES_DISTANCE) * i);
            pipes.push_back(pipe);
        }
        Pipes { pipes }
    }

    pub fn reset(&mut self, ctx: &mut RaylibDrawHandle) {
        let mut pipes: VecDeque<Pipe> = VecDeque::new();
        let mid = (ctx.get_screen_width().as_f32() * 0.5) as i32;
        for i in 0..5 {
            let pipe = Pipe::with_x_dh(ctx, mid + (PIPES_DISTANCE) * i);
            pipes.push_back(pipe);
        }
        self.pipes = pipes;
    }

    pub fn draw(&mut self, ctx: &mut RaylibDrawHandle) {
        for pipe in self.pipes.iter_mut() {
            pipe.draw(ctx);
        }
    }

    pub fn update(&mut self, ctx: &mut RaylibDrawHandle) {
        if self.pipes[0].get_x() < -60 {
            self.remove_first();
            self.add_last(ctx);
        }
    }

    pub fn remove_first(&mut self) {
        self.pipes.pop_front();
    }

    pub fn add_last(&mut self, ctx: &mut RaylibDrawHandle) {
        let l = self.pipes.len();
        let pipe = Pipe::with_x_dh(ctx, self.pipes[l - 1].get_x() + PIPES_DISTANCE as i32);
        self.pipes.push_back(pipe);
    }
}

impl Index<usize> for Pipes {
    type Output = Pipe;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pipes[index]
    }
}

impl IndexMut<usize> for Pipes {
    fn index_mut(&mut self, index: usize) -> &mut Pipe {
        &mut self.pipes[index]
    }
}
