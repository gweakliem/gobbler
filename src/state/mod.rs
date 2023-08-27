//! State contains all the game state and logic.
use bracket_lib::prelude::*;

const GAME_WINDOW_HEIGHT: isize = 400;
const GAME_WINDOW_WIDTH: isize = 640;
const BOX_HEIGHT: isize = 132;
const BOX_WIDTH: isize = 85;
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

struct Point {
    x: i32,
    y: i32,
}

pub struct State {
    box_y: isize, // Oscar's vertical position.
    box_x: isize,
    box_moving: Moving, // Direction the box is moving.
    food: Vec<Point>,
    rng: RandomNumberGenerator,
    timer: f32,
}

pub fn new(rng: RandomNumberGenerator) -> State {
    return State {
        box_y: FLOOR_COLLISION-1,
        box_x: (GAME_WINDOW_WIDTH / 2) - (BOX_WIDTH / 2),
        box_moving: Moving::Not,
        food: Vec::new(),
        rng,
        timer: 0.0,
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
        // 1 is with_simple_console_no_bg, the debug console
        bterm.set_active_console(1);
        bterm.cls();
        bterm.print(1, 1, "Watch them go!");
        bterm.printer(
            1,
            2,
            &format!("#[pink]FPS: #[]{} fatty {} {}", bterm.fps, self.box_x, self.box_y),
            TextAlign::Left,
            None,
        );

        // 0 is the sprite console
        bterm.set_active_console(0);
        bterm.cls();

        bterm.add_sprite(
            Rect::with_size(self.box_x, self.box_y, 32, 32),
            400, // - self.box_y,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            0, //self.frame % 4,
        );

        for dood in self.food.iter() {
            bterm.add_sprite(
                Rect::with_size(dood.x, dood.y, 32, 32),
                400 - dood.y,
                RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                0, //self.frame % 4,
            )
        }

        self.timer += bterm.frame_time_ms;
        if self.timer > 66.0 {
            self.timer = 0.0;
            //self.frame += 1;

            // for dood in self.food.iter_mut() {
            //     dood.x += self.rng.range(0, 3) - 1;
            //     dood.y += self.rng.range(0, 3) - 1;
            // }
        }

/*
        bterm.draw_box_double(
            self.box_x, // x
            self.box_y, // y
            BOX_WIDTH,  // width
            BOX_WIDTH,  // height
            RED,        // foreground color
            RED,        // background color
        );
*/

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
