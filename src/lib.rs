use block_id::{Alphabet, BlockId};
use rand::{thread_rng, Rng};
use std::cmp::Eq;

pub mod entity;
use crate::entity::{Entity, Personality, Player};

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

// TODO::Implement scoring types as enums?
// TODO::Handle other types like algebraic (ie "P","R","S","T")
impl Outcome {
    fn positive_scoring(o: &Outcome) -> i32 {
        match o {
            Outcome::PUNISH => 0,
            Outcome::SUCKER => -1,
            Outcome::REWARD => 2,
            Outcome::TEMPTATION => 3,
        }
    }
}

// Choice represents the two choices of the game
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    CHEAT,
    COOPERATE,
}

// make_players will assemble a Vector of basic entities
pub fn make_players(num: i32) -> Vec<entity::Entity> {
    let mut player_gen = Vec::new();
    let mut rng = thread_rng();
    let length = 3;
    let seed = 0o152;
    let generator = BlockId::new(Alphabet::alphanumeric(), seed, length);
    for _ in 0..num {
        let tmp = Entity::new(rng.gen::<Personality>(), generator.encode_string(rng.gen()));
        // let tmp = Entity::new_player(rng.gen::<Personality>(), i.to_string());
        player_gen.push(tmp);
    }
    player_gen
}

// set_rounds takes a Vec of players and produces the order they will compete
// against each other, it is called at the beginning of round_robin
fn set_rounds(players: &Vec<impl entity::Player>) -> Vec<(String, String)> {
    let mut round_list = Vec::new();
    let mut opponents = players.clone();
    for player in players {
        opponents.retain(|opp| opp.name() != player.name());
        opponents
            .iter()
            .for_each(|o| round_list.push((player.tag().to_string(), o.tag().to_string())));
    }
    round_list
}

// play_game determines what kind of game to play
// TODO::more modes
pub fn play_game(players: &mut Vec<impl entity::Player>, rounds: i32) {
    if rounds <= 1 {
        play_round_robin(players)
    } else {
        play_tournament(players, rounds)
    }
}

fn play_tournament(players: &mut Vec<impl entity::Player>, rounds: i32) {
    for _ in 0..rounds {
        play_round_robin(players);
    }
}

fn play_round_robin(players: &mut Vec<impl entity::Player>) {
    let rounds = set_rounds(players);
    for (p1, p2) in &rounds {
        let (c1, c2);
        {
            let player_one = find(p1, players).unwrap();
            let player_two = find(p2, players).unwrap();
            c1 = player_one.choose(player_two.tag());
            c2 = player_two.choose(player_one.tag());
        }
        find_mut(p1, players).unwrap().add_memory(p2, (c1, c2));
        find_mut(p2, players).unwrap().add_memory(p1, (c2, c1));
    }
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

//
// Things that should probably be generics
//

fn find<'a>(tag: &str, players: &'a Vec<impl entity::Player>) -> Option<&'a impl entity::Player> {
    for (_, v) in players.iter().enumerate() {
        if v.tag() == tag {
            return Some(v);
        }
    }
    None
}

fn find_mut<'a>(
    tag: &str,
    players: &'a mut Vec<impl entity::Player>,
) -> Option<&'a mut impl entity::Player> {
    for (_, v) in players.iter_mut().enumerate() {
        if v.tag() == tag {
            return Some(v);
        }
    }
    None
}
