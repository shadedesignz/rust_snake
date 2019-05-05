use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::Snake;
use crate::draw::draw_block;

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];

pub struct Food {
    x: i32,
    y: i32,
}

impl Food {
    pub fn new(x: i32, y: i32) -> Food {
        Food {
            x,
            y,
        }
    }

    pub fn add_food(&mut self, snake: &Snake, game_width: i32, game_height: i32) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, game_width - 1);
        let mut new_y = rng.gen_range(1, game_height - 1);
        while snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, game_width - 1);
            new_y = rng.gen_range(1, game_height - 1);
        }

        self.x = new_x as i32;
        self.y = new_y as i32;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(FOOD_COLOR, self.x, self.y, con, g);
    }

    pub fn check_collision(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}