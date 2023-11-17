//! An example game can be expressed using the library types:
//!
//! ```rust,no_run
//! let mut players = prisoner::make_players(args.players);
//! prisoner::play_game(&mut players, args.rounds.unwrap_or(1));
//! let output_table = Table::new(players)
//!     .with(Style::rounded())
//!     // .with(Modify::new(Rows::single(1)).with(Border::default().top('x')))
//!     .to_string();
//! print!("{}", output_table)
//! ```
//!
//! Providing more than one round initiates a "tournament", where round-robin
//! will occur per number of rounds.
//!
use block_id::{Alphabet, BlockId};
use rand::{thread_rng, Rng};
use std::cmp::Eq;

pub mod entity;
use crate::entity::{Entity, Personality};

/// Outcome represents results of the game. There can only be these four
/// results. Different scoring implementations of functions can be applied.
///
/// The scoring equation is `T > R > P > S`.
#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    /// The result if both players CHEAT
    PUNISH,
    /// The result if a player COOPERATES, and their opponent CHEATs
    SUCKER,
    /// The result if both players COOPERATE
    REWARD,
    /// The result the player CHEATs and their opponents COOPERATEs
    TEMPTATION,
}

impl Outcome {
    /// Traditional scoring
    pub fn traditional(o: &Outcome) -> i32 {
        match o {
            Outcome::PUNISH => -2,
            Outcome::SUCKER => -3,
            Outcome::REWARD => -1,
            Outcome::TEMPTATION => 0,
        }
    }

    /// A "positive" scoring system
    pub fn positive(o: &Outcome) -> i32 {
        match o {
            Outcome::PUNISH => 0,
            Outcome::SUCKER => -1,
            Outcome::REWARD => 2,
            Outcome::TEMPTATION => 3,
        }
    }

    /// Get results as symbols (ie "`T, R, P, S`")
    pub fn algebraic(o: &Outcome) -> char {
        match o {
            Outcome::PUNISH => 'P',
            Outcome::SUCKER => 'S',
            Outcome::REWARD => 'R',
            Outcome::TEMPTATION => 'T',
        }
    }
}

/// In every roound the player can only make one of two choices, CHEAT, or COOPERATE
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    /// Attempt to betray the other player
    CHEAT,
    /// Attempt to work with the other player
    COOPERATE,
}

/// make_players will assemble a Vector of basic entities using fake data
/// and a random distribution of personalities
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

/// At the heart of the prisoners dilemma is the choice between two players
/// they can choose to COOPERATE or CHEAT (or BETRAY, etc). The possible outcomes
/// can be found here: https://en.wikipedia.org/wiki/Prisoner%27s_dilemma
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
