#[macro_use]
extern crate prettytable;
use prettytable::{format, Cell as tCell, Row, Table};
use std::cmp::Eq;
use std::fmt;

pub mod entity;
pub mod game;

use entity::PlayerFactory;

pub const TITLE: &'static str = "The prisoners dilemna";

// Outcome is an enum to express the reward values of the game result matrix
// TODO::return the classic T > R > P > S representation and provide a trait
// to implement the reward values
#[derive(Copy, Clone)]
pub enum Outcome {
    PUNISH = 0,
    SUCKER = -1,
    REWARD = 2,
    TEMPTATION = 3,
}

impl Outcome {
    pub fn as_i16(&self) -> i16 {
        *self as i16
    }
}

// Choice represents the two choices of the game
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Choice {
    CHEAT,
    COOPERATE,
}

// play_game determines what kind of game to play
// TODO::more modes
pub fn play_game(mut game: game::Game, rounds: i16) {
    let players = game.get_players();
    if rounds == 0 {
        play_round_robin(players);
    } else {
        play_tournament(players, rounds)
    }

    print_result(players);
}

// play_tournament will pit every member of a group against each other in
// a round robin n number of times
fn play_tournament(players: &mut Vec<Box<dyn entity::Player>>, rounds: i16) {
    for _ in 0..rounds {
        play_round_robin(players);
    }
}

// play_round_robin will pit every member of a group against each other once
fn play_round_robin(players: &mut Vec<Box<dyn entity::Player>>) {
    for i in 0..players.len() {
        if i + 1 == players.len() {
            return;
        }
        if players[i].get_name() == players[i + 1].get_name() {
            continue;
        }
        play_round(&players[i], &players[i + 1])
    }
}

// At the heart of the prisoners dilemma is the choice between two players
// they can choose to COOPERATE or CHEAT (or BETRAY, etc). The possible outcomes
// can be found here: https://en.wikipedia.org/wiki/Prisoner%27s_dilemma
fn determine(m1: &Choice, m2: &Choice) -> (Outcome, Outcome) {
    match m1 {
        Choice::COOPERATE => {
            if m1 == m2 {
                (Outcome::REWARD, Outcome::REWARD)
            } else {
                (Outcome::SUCKER, Outcome::TEMPTATION)
            }
        }
        Choice::CHEAT => {
            if m1 == m2 {
                (Outcome::PUNISH, Outcome::PUNISH)
            } else {
                (Outcome::TEMPTATION, Outcome::SUCKER)
            }
        }
    }
}

pub fn play_round(player_one: &Box<dyn entity::Player>, player_two: &Box<dyn entity::Player>) {
    let m1 = player_one.choose();
    let m2 = player_two.choose();

    let (o1, o2) = determine(&m1, &m2);

    player_one.get_entity().record_result(o1);
    player_two.get_entity().record_result(o2);
}

fn print_outcome(
    player_one: &Box<dyn entity::Player>,
    player_two: &Box<dyn entity::Player>,
    o1: Outcome,
    o2: Outcome,
) {
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["Name", "Behavior", "Result"]);
    table.add_row(Row::new(vec![
        tCell::new(&player_one.get_name()),
        tCell::new(&player_one.get_behavior()),
        tCell::new(&o1.to_string()),
    ]));
    table.add_row(Row::new(vec![
        tCell::new(&player_two.get_name()),
        tCell::new(&player_two.get_behavior()),
        tCell::new(&o2.to_string()),
    ]));
    table.printstd();
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

pub fn print_result(players: &Vec<Box<dyn entity::Player>>) {
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["Behavior", "Name", "Score"]);
    for p in players {
        table.add_row(Row::new(vec![
            tCell::new(&p.get_behavior()),
            tCell::new(&p.get_name()),
            tCell::new(&p.get_entity().get_score().to_string()),
        ]));
    }

    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.printstd();
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
            Outcome::PUNISH => "Punish",
            Outcome::SUCKER => "Sucker",
            Outcome::REWARD => "Reward",
            Outcome::TEMPTATION => "Temptation",
        };
        write!(f, "{}", printable)
    }
}
