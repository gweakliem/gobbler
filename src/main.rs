use bracket_lib::{terminal::{BTermBuilder, main_loop, EMBED}, random::RandomNumberGenerator};
use bracket_terminal::prelude::{SpriteSheet, Rect};

mod state;

bracket_terminal::embedded_resource!(FATTY_CAT, "resources/fattysprite.png");

fn main() {
    bracket_terminal::link_resource!(FATTY_CAT, "resources/fattysprite.png");

    let context = BTermBuilder::new()
        .with_sprite_console(640, 400, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console_no_bg(80, 50, "terminal8x8.png")
        .with_title("Gobbler")
        .with_sprite_sheet(
            SpriteSheet::new("resources/fattysprite.png")
                .add_sprite(Rect::with_size(0, 0, 85, 132)))
        .with_vsync(false)
        .build();

    match context {
        Ok(bterm) => {
            let _ = main_loop(bterm, state::new(RandomNumberGenerator::new()));
        }
        Err(err) => {
            println!("An error has occurred: {err:?}");
            println!("Aborting");
        }
    }
}
