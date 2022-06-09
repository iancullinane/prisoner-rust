use clap::Parser;
use tabled::{Style, Table};
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
    rounds: Option<i32>,
}

fn main() {
    // clap
    let args = Args::parse();

    let mut players = prisoner::make_players(args.players);
    prisoner::play_game(&mut players, args.rounds.unwrap_or(1));
    let output_table = Table::new(players).with(Style::rounded()).to_string();
    print!("{}", output_table)
}
