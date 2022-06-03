use rand::{thread_rng, Rng};
use std::borrow::BorrowMut;
use std::cmp::Eq;
use std::fmt;
use std::iter;

pub mod entity;
pub mod game;

use entity::Personality;

use crate::entity::Entity;

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
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

// Choice represents the two choices of the game
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    CHEAT,
    COOPERATE,
}

pub fn new_game(players: i32) -> game::Game<Entity> {
    println!("New game for {} players", players);
    let mut rng = thread_rng();
    let mut new_game: game::Game<Entity> = game::Game {
        name: String::from("Test Game"),
        players: vec![],
    };

    for _ in 0..players {
        let tmp = Entity::new_player(rng.gen::<Personality>());
        new_game.add_player(tmp);
    }
    new_game
}

// play_game determines what kind of game to play
// TODO::more modes
pub fn play_game<E: entity::Player>(mut game: game::Game<E>, rounds: i16) {
    let players = game.get_players();
    play_round_robin(players);
    // print_result(players);
}

pub fn play_round_robin(players: Vec<impl entity::Player>) {
    let mut opp = players.clone();

    for p in players.iter() {
        println!("{}", p)
    }
    // for p in players {
    //     println!("{}", p)
    // }

    //
    //

    // let cp = iter::once(players);
    // match cp.next().unwrap() {
    //     Some() -> println!("some"),
    //     None() -> println!("some"),
    // }
    // cp.for_each(|player| {
    //     println!("Player: {}", player);
    // });

    // for p in players.iter(). {
    //     for opp in players.iter() {
    //         println!("{} plays {}", p.get_name(), opp.get_name())
    //     }
    // players.retain(|&i| i.get_name() != p.get_name());
    // players.retain(|x| x.get_name() == "steve")
    // }
    // for i in 0..players. len() {
    //     for k in i..players.len() {

    //         once(players[i], players[k])
    //     }
    // }
}

pub fn once(player_one: &mut dyn entity::Player, player_two: &mut dyn entity::Player) {
    let m1 = player_one.choose();
    let m2 = player_two.choose();

    let (o1, o2) = determine(m1, m2);

    player_one.record_result(o1);
    player_two.record_result(o2);
}

// At the heart of the prisoners dilemma is the choice between two players
// they can choose to COOPERATE or CHEAT (or BETRAY, etc). The possible outcomes
// can be found here: https://en.wikipedia.org/wiki/Prisoner%27s_dilemma
fn determine(m1: Choice, m2: Choice) -> (Outcome, Outcome) {
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

pub fn print_result(players: &[impl entity::Player]) {
    for p in players.iter() {
        println!("{}", p)
    }
}
