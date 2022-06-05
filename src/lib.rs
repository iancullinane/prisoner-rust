use block_id::{Alphabet, BlockId};
use rand::{thread_rng, Rng};
use std::cmp::Eq;

pub mod entity;
pub mod game;

use entity::Personality;

use crate::entity::Entity;
use crate::entity::Player;

// Outcome is an enum to express the reward values of the game result matrix
// TODO::return the classic T > R > P > S representation and provide a trait
// to implement the reward values
#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    PUNISH,
    SUCKER,
    REWARD,
    TEMPTATION,
}

impl Outcome {
    fn positive_scoring(o: Outcome) -> i8 {
        match o {
            Outcome::PUNISH => 0,
            Outcome::SUCKER => -1,
            Outcome::REWARD => 2,
            Outcome::TEMPTATION => 3,
        }
    }
}

// impl Outcome {
//     pub fn as_i32(&self) -> i32 {
//         *self as i32
//     }
// }

// Choice represents the two choices of the game
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    CHEAT,
    COOPERATE,
}

pub fn make_players(num: i32) -> Vec<entity::Entity> {
    let mut player_gen = Vec::new();
    let mut rng = thread_rng();
    let length = 5;
    let seed = 0o152;
    let generator = BlockId::new(Alphabet::alphanumeric(), seed, length);
    for _ in 0..num {
        let tmp = Entity::new_player(rng.gen::<Personality>(), generator.encode_string(rng.gen()));
        player_gen.push(tmp);
    }
    player_gen
}

// play_game determines what kind of game to play
// TODO::more modes
pub fn play_game(players: Vec<impl entity::Player>, _rounds: i16) {
    play_round_robin(players);
}

pub fn play_round_robin(players: Vec<impl entity::Player>) {
    let mut opponents = players.clone();
    let mut game_log = Vec::<(Outcome, Outcome)>::new();
    for player in players {
        opponents.retain(|opp| opp.get_name() != player.get_name());
        // opponents.iter().for_each(|o| once(player, o));
        opponents.iter().for_each(|o| game_log.push(player.play(o)));
    }

    for l in &game_log {
        println! {"{:?}", l}
    }
}

pub fn once(player_one: impl Player, player_two: impl Player) {
    let m1 = player_one.choose();
    let m2 = player_two.choose();

    let outcome = determine(m1, m2);

    // player_one.score(outcome.0);
    // player_two.score(outcome.1);
}

// At the heart of the prisoners dilemma is the choice between two players
// they can choose to COOPERATE or CHEAT (or BETRAY, etc). The possible outcomes
// can be found here: https://en.wikipedia.org/wiki/Prisoner%27s_dilemma
pub fn determine(m1: Choice, m2: Choice) -> (Outcome, Outcome) {
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

// pub fn new_game(players: i32) -> game::Game<Entity> {
//     // println!("New game for {} players", players);
//     let mut rng = thread_rng();
//     let mut new_game: game::Game<Entity> = game::Game {
//         name: String::from("Test Game"),
//         players: vec![],
//     };

//     for _ in 0..players {
//         let tmp = Entity::new_player(rng.gen::<Personality>());
//         new_game.add_player(tmp);
//     }
//     new_game
// }
