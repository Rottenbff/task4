mod args;
mod fair_random;
mod game;
mod morty;
mod stats;

use crate::{
    args::CliArgs,
    game::Game,
    morty::{ClassicMorty, LazyMorty, Morty},
};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    let morty: Box<dyn Morty> = match args.morty_name.as_str() {
        "ClassicMorty" => Box::new(ClassicMorty),
        "LazyMorty" => Box::new(LazyMorty),
        _ => {
            eprintln!(
                "Error: Unknown Morty implementation '{}'. Available options: ClassicMorty, LazyMorty.",
                args.morty_name
            );
            eprintln!("\nExample: cargo run 5 ClassicMorty");
            std::process::exit(1);
        }
    };

    let mut game = Game::new(args.boxes, morty);
    game.run();
}
