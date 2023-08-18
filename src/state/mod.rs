//! State contains all the game state and logic.
use bracket_lib::prelude::*;

const GAME_WINDOW_HEIGHT: isize = 50;
const GAME_WINDOW_WIDTH: isize = 80;
const BOX_HEIGHT: isize = 5;
const BOX_WIDTH: isize = 5;
const CEILING_POS: isize = 5;
const FLOOR_POS: isize = GAME_WINDOW_HEIGHT - BOX_HEIGHT;
const LEFT_WALL_POS: isize = 5;
const RIGHT_WALL_POS: isize = GAME_WINDOW_WIDTH - BOX_WIDTH;
const CEILING_COLLISION: isize = CEILING_POS + 1;
const FLOOR_COLLISION: isize = FLOOR_POS - BOX_HEIGHT - 1;
const LEFT_WALL_COLLISION: isize = LEFT_WALL_POS + 1;
const RIGHT_WALL_COLLISION: isize = RIGHT_WALL_POS - BOX_HEIGHT - 1;

enum Moving {
    Not,
    Up,
    Right,
    Down,
    Left,
}

pub struct State {
    box_y: isize, // Box's vertical position.
    box_x: isize,
    box_moving: Moving, // Direction the box is moving.
}

pub fn new() -> State {
    return State {
        box_y: FLOOR_COLLISION,
        box_x: (GAME_WINDOW_WIDTH / 2) - 3,
        box_moving: Moving::Not,
    };
}

/// State implementation of the GameState trait.
impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        self.keyboard_input(bterm);
        self.render(bterm);
    }
}

/// Method set for the State type.
impl State {
    /// keyboard_input handles the processing of keyboard input.
    fn keyboard_input(&mut self, bterm: &mut BTerm) {
        match bterm.key {
            None => {}
            Some(VirtualKeyCode::Space) => {
                self.box_moving = Moving::Not;
            }
            Some(VirtualKeyCode::Up) => {
                self.box_moving = Moving::Up;
            }
            Some(VirtualKeyCode::Down) => {
                self.box_moving = Moving::Down;
            }
            Some(VirtualKeyCode::Right) => {
                self.box_moving = Moving::Right;
            }
            Some(VirtualKeyCode::Left) => {
                self.box_moving = Moving::Left;
            }
            _ => {}
        };
    }

    /// render takes the current game state and renders the screen.
    fn render(&mut self, bterm: &mut BTerm) {
        bterm.cls_bg(WHITE);
        bterm.draw_bar_horizontal(
            0,                  // x
            CEILING_POS,        // y
            GAME_WINDOW_WIDTH,  // width
            GAME_WINDOW_HEIGHT, // n
            GAME_WINDOW_HEIGHT, // max
            YELLOW,             // foreground color
            YELLOW,             // background color
        );
        bterm.draw_bar_horizontal(
            0,                  // x
            FLOOR_POS,          // y
            GAME_WINDOW_WIDTH,  // width
            GAME_WINDOW_HEIGHT, // n
            GAME_WINDOW_HEIGHT, // max
            YELLOW,             // foreground color
            YELLOW,             // background color
        );

        bterm.draw_bar_vertical(
            LEFT_WALL_POS,      // x
            0,                  // y
            GAME_WINDOW_WIDTH,  // width
            GAME_WINDOW_HEIGHT, // n
            GAME_WINDOW_HEIGHT, // max
            YELLOW,             // foreground color
            YELLOW,             // background color
        );

        bterm.draw_bar_vertical(
            RIGHT_WALL_POS,     // x
            0,                  // y
            GAME_WINDOW_WIDTH,  // width
            GAME_WINDOW_HEIGHT, // n
            GAME_WINDOW_HEIGHT, // max
            YELLOW,             // foreground color
            YELLOW,             // background color
        );

        bterm.draw_box_double(
            self.box_x, // x
            self.box_y, // y
            BOX_WIDTH,  // width
            BOX_WIDTH,  // height
            RED,        // foreground color
            RED,        // background color
        );

        match self.box_moving {
            Moving::Down => {
                self.box_y += 1;
                if self.box_y == FLOOR_COLLISION {
                    self.box_moving = Moving::Up;
                }
            }
            Moving::Up => {
                self.box_y -= 1;
                if self.box_y == CEILING_COLLISION {
                    self.box_moving = Moving::Down;
                }
            }
            Moving::Right => {
                self.box_x += 1;
                if self.box_x == RIGHT_WALL_COLLISION {
                    self.box_moving = Moving::Left;
                }
            }
            Moving::Left => {
                self.box_x -= 1;
                if self.box_x == LEFT_WALL_COLLISION {
                    self.box_moving = Moving::Right;
                }
            }
            Moving::Not => {}
        }
    }
}
