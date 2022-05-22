use fake::faker::name::en::*;
use fake::Fake;
use rand::Rng;
use std::cell::Cell;
use std::fmt;

use tabled::{Table, Tabled};

use crate::Choice;
use crate::Outcome;

use rand::distributions::{Distribution, Standard};

// Define the types of personalities, actual decisions handled by `choose()`
#[derive(Clone, Copy, Debug)]
pub enum Personality {
    AlwaysCooperate,
    AlwaysCheat,
    CopyCat,
    Vengeful,
    SlowLearner,
}

// To allow the use of `rand` traits on Personality
impl Distribution<Personality> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Personality {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..5) {
            // rand 0.8
            0 => Personality::AlwaysCooperate,
            1 => Personality::AlwaysCheat,
            2 => Personality::CopyCat,
            3 => Personality::Vengeful,
            4 => Personality::SlowLearner,
            _ => Personality::AlwaysCooperate,
        }
    }
}

// A concrete struct from which to base the Player trait and hold their data
#[derive(Clone, Debug, Tabled)]
pub struct Entity {
    // TODO::Generate short id's https://github.com/drifting-in-space/block-id
    name: String,
    score: i32,
    memory: Memory,
    personality_type: Personality,
}

// https://github.com/mre/idiomatic-rust
// https://github.com/brson/rust-anthology/tree/master/src

impl Entity {
    pub fn new(p: Personality) -> Self {
        Self {
            name: FirstName().fake(),
            score: 0,
            memory: Memory::new(),
            personality_type: p,
        }
    }

    pub fn new_player(p: Personality) -> Entity {
        Entity::new(p)
    }

    // pub fn get_name(&self) -> String {
    //     self.name.to_string()
    // }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn get_personality_type(&self) -> &Personality {
        &self.personality_type
    }
}

// Choose parses all personalities
fn choose(p: &Personality, m: &Memory) -> Choice {
    match p {
        Personality::AlwaysCooperate => Choice::COOPERATE,
        Personality::AlwaysCheat => Choice::CHEAT,
        Personality::CopyCat => m.opp_last_move,
        Personality::Vengeful => {
            if m.betrayed.get() > 1 {
                return Choice::CHEAT;
            }
            Choice::COOPERATE
        }
        Personality::SlowLearner => {
            if m.betrayed.get() > 5 {
                return Choice::CHEAT;
            }
            Choice::COOPERATE
        }
    }
}

// Player is the trait to repesent a player of the game
// most notably the players behavior implementation
pub trait Player: fmt::Display {
    fn choose(&self) -> Choice;
    fn record_result(&mut self, o: Outcome);
    fn get_name(&self) -> &str;
}

impl Player for Entity {
    fn choose(&self) -> Choice {
        choose(&self.personality_type, &self.memory)
    }

    fn record_result(&mut self, o: Outcome) {
        self.score += o.as_i32();
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

// Memory is used when making a choice on how to play
// TODO::implement a lot more functions on memory
#[derive(Clone, Debug)]
pub struct Memory {
    opp_last_move: Choice,
    last_move: Choice,
    betrayed: Cell<i16>,
}

impl Memory {
    fn new() -> Self {
        Self {
            opp_last_move: Choice::COOPERATE, // everyone starts nice
            last_move: Choice::COOPERATE,     // everyone starts nice
            betrayed: Cell::new(0),
        }
    }
}

fn reverse(c: Choice) -> Choice {
    match c {
        Choice::COOPERATE => Choice::CHEAT,
        Choice::CHEAT => Choice::COOPERATE,
    }
}
// For printing outcomes
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!(
            "{}\t{}\t{}",
            self.get_name(),
            self.get_score(),
            self.get_personality_type()
        );
        write!(f, "{}", output)
    }
}

impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "{}", String::from("Memories")}
    }
}
