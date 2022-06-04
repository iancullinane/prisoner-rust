use clap::Parser;
use rand::{thread_rng, Rng};

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
    let args = Args::parse();
    let players = prisoner::make_players(args.players);

    // let players: Vec<entity::Entity> = (0..args.players)
    //     .map(|_x| Entity::new(rng.gen::<Personality>()))
    //     .collect();

    // for _ in 0..args.players {
    //     let tmp = Entity::new_player(rng.gen::<Personality>());
    //     // new_game.add_player(tmp);
    // }
    // let game = prisoner::new_game(args.players);
    // println!("{}", game.get_name());
    // prisoner::play_game(game, args.rounds.unwrap_or(0));
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
