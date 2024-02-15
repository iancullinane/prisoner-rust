use clap::Parser;
use prisoner::entity;
use std::io;
use tabled::{Style, Table};
// https://brson.github.io/rust-anthology/1/effectively-using-iterators.html
// https://github.com/mre/idiomatic-rust
// https://github.com/brson/rust-anthology/tree/master/src
// https://github.com/rustomax/rust-iterators

// https://quickref.me/rust.html

// TODO::https://www.nature.com/articles/s41598-018-20426-w
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    players: i32,

    #[clap(short, long)]
    rounds: Option<i32>,
}

pub mod game;

fn main() {
    // ...
    let args = Args::parse();
    let mut players = game::make_players(args.players);
    let output_table = Table::new(players.clone())
        .with(Style::rounded())
        .to_string();
    print!("{}", output_table);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            // "findplayer" => {
            //     if parts.len() > 1 {
            //         let player = entity::find(|&p: &Player| p.name() == parts[1], &players);
            //         println!("{}", player.full_info());
            //         // match entity::find_by_name(parts[1], &players) {
            //         //     Some(player) => println!("{}", player.full_info()),
            //         //     None => println!("Player not found"),
            //         // }
            //     } else {
            //         println!("Please provide a player name.");
            //     }
            // }

            // "history" => {
            //     if parts.len() > 1 {
            //         let player = entity::find(|&&p| p.name() == parts[1], &players);
            //         println!("Please provide a player name.");
            //     }
            // }
            "play" => {
                game::play_game(&mut players, args.rounds.unwrap_or(1));
                let output_table = Table::new(&players).with(Style::rounded()).to_string();
                print!("{}", output_table)
            }

            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}
