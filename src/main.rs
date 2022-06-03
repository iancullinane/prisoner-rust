use clap::Parser;

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

    let game = prisoner::new_game(args.players);
    println!("{}", game.get_name());
    prisoner::play_game(game, args.rounds.unwrap_or(0));
}
