use clap::Parser;
use tabled::Table;
// use crate::entity::{Entity, Player};
// https://brson.github.io/rust-anthology/1/effectively-using-iterators.html
// https://github.com/mre/idiomatic-rust
// https://github.com/brson/rust-anthology/tree/master/src
// https://github.com/rustomax/rust-iterators

// TODO::https://www.nature.com/articles/s41598-018-20426-w
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    players: i32,

    #[clap(short, long)]
    rounds: Option<i16>,
}

fn main() {
    // clap
    let args = Args::parse();

    let mut players = prisoner::make_players(args.players);
    prisoner::play_game(&mut players, args.rounds.unwrap_or(1));
    let output_table = Table::new(players).to_string();
    print!("{}", output_table)
}

// fn make_entities(n: i32) -> Vec<entity::Entity> {
//     let v: Vec<entity::Entity> = (0..n).map(|_x| Entity::new()).collect();
//     v
// }
// let alphabet = Alphabet::alphanumeric();

// // The generator takes a u128 as a seed.
// let seed = 1234;

// // The length of a generated code. This is really a _minimum_ length; larger numbers
// // will be converted to longer codes since that's the only way to avoid collisions.
// let length = 4;

// // A small amount of pre-caching work happens when we create the BlockId instance,
// // so it's good to re-use the same generator where possible.
// let generator = BlockId::new(alphabet, seed, length);
