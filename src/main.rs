use clap::Parser;
use prisoner;
// use
// use prisoner::TITLE;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    players: i32,

    #[clap(short, long)]
    rounds: i16,
    // #[clap(short, long, default_value_t = 1)]
    // count: u8,
}

fn main() {
    let args = Args::parse();

    let game = prisoner::new_game(args.players);
    println!("{}", game.get_name());
    prisoner::play_tournament(game, args.rounds);
    // game.play_tournament()
    // for e in game.players {
    //     println!("{}", e)
    // }
}
