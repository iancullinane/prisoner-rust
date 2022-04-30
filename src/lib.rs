// use prisoner::game;

use std::cmp::Eq;
use std::fmt;

pub const TITLE: &'static str = "The prisoners dilemna";

pub mod entity;
pub mod game;

use entity::PlayerFactory;

#[derive(Copy, Clone)]
pub enum Outcome {
    NOTHING = 0,
    SUCKER = -1,
    REWARD = 2,
    TEMPTATION = 3,
}

impl Outcome {
    pub fn as_i16(&self) -> i16 {
        *self as i16
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Choice {
    CHEAT,
    COOPERATE,
}

pub fn play_tournament(mut game: game::Game, rounds: i16) {
    println!("Tournament mode:");
    let players = game.get_players();
    for _ in 0..rounds {
        play_round_robin(players);
    }

    for p in players {
        println!("[{}] {}", p.get_behavior(), p.get_entity())
    }
}

fn play_round_robin(players: &mut Vec<Box<dyn entity::Player>>) {
    for i in 0..players.len() {
        for j in 0..players.len() {
            if players[i].get_name() == players[j].get_name() {
                continue;
            }
            play_round(&players[i], &players[j])
        }
    }
}

fn determine(m1: &Choice, m2: &Choice) -> Outcome {
    match m1 {
        Choice::COOPERATE => {
            if m1 == m2 {
                Outcome::REWARD
            } else {
                Outcome::SUCKER
            }
        }
        Choice::CHEAT => {
            if m1 == m2 {
                Outcome::NOTHING
            } else {
                Outcome::TEMPTATION
            }
        }
    }
}

pub fn play_round(player_one: &Box<dyn entity::Player>, player_two: &Box<dyn entity::Player>) {
    let m1 = player_one.choose();
    let m2 = player_two.choose();

    let o1 = determine(&m1, &m2);
    let o2 = determine(&m2, &m1);

    print!("{} vs {}\n", player_one.get_name(), player_two.get_name());
    print!("{} : {}\n", o1.as_i16(), o2.as_i16());

    player_one.get_entity().record_result(o1);
    player_two.get_entity().record_result(o2);
}

pub fn new_game(players: i32) -> game::Game {
    let mut r = game::Game {
        name: "Test Game".to_string(),
        players: Vec::new(),
    };

    for _ in 0..players {
        let tmp = PlayerFactory::get_player();
        r.players.push(tmp)
    }

    r
}

// For printing outcomes
impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Choice::CHEAT => 'X',
            Choice::COOPERATE => '0',
        };
        write!(f, "{}", printable)
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Outcome::NOTHING => "Nothing",
            Outcome::SUCKER => "Sucker",
            Outcome::REWARD => "Reward",
            Outcome::TEMPTATION => "Temptation",
        };
        write!(f, "{}", printable)
    }
}
