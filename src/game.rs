use piston_window::*;
use piston_window::types::Color;

use crate::snake::{Direction, Snake};
use crate::draw::{draw_rectange, to_coord, BLOCK_SIZE};
use crate::food::Food;

const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
const GAMEPAUSE_COLOR: Color = [0.00, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.15;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food: Food,
    food_exists: bool,

    width: i32,
    height: i32,

    game_over: bool,
    game_paused: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food: Food::new(6, 4),
            food_exists: true,
            width,
            height,
            game_over: false,
            game_paused: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        if key == Key::P {
            self.game_paused = !self.game_paused;
            return;
        }

        let dir = match key {
            Key::Up | Key::W => Some(Direction::Up),
            Key::Down | Key::S => Some(Direction::Down),
            Key::Left | Key::A => Some(Direction::Left),
            Key::Right | Key::D => Some(Direction::Right),
            _ => None,
        };

        if dir.is_none() || dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            self.food.draw(con, g);
        }

        let border = rectangle::Rectangle::new_border(BORDER_COLOR, BLOCK_SIZE);
        border.draw(
            [0.0, 0.0, to_coord(self.width), to_coord(self.height)],
            &draw_state::DrawState::default(),
            con.transform,
            g 
        );

        if self.game_over {
            draw_rectange(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

        if self.game_paused {
            draw_rectange(GAMEPAUSE_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.food.add_food(&self.snake, self.width, self.height);
            self.food_exists = true;
        }

        if self.waiting_time > MOVING_PERIOD && !self.game_paused {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food.check_collision(head_x, head_y) {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food = Food::new(6, 4);
        self.game_over = false;
    }
}