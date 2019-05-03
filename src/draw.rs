use piston_window::{rectangle, Context, G2d, draw_state};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let rect = rectangle::Rectangle::new(color);
    let border = rectangle::Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 1.0);
    border.draw(
       [gui_x + 0.5, gui_y + 0.5, BLOCK_SIZE - 0.5, BLOCK_SIZE - 0.5],
        &draw_state::DrawState::default(),
        con.transform,
        g 
    );
    rect.draw(
        [gui_x + 0.5, gui_y + 0.5, BLOCK_SIZE - 0.5, BLOCK_SIZE - 0.5],
        &draw_state::DrawState::default(),
        con.transform,
        g
    );
}

pub fn draw_rectange(
    color: Color,
    x: i32,
    y: i32, 
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}