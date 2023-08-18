use bracket_lib::terminal::{BTermBuilder, main_loop};
mod state;

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Bracket World").build();

    match context {
        Ok(bterm) => {
            let _ = main_loop(bterm, state::new());
        }
        Err(err) => {
            println!("An error has occurred: {err:?}");
            println!("Aborting");
        }
    }
}
