use fake::faker::name::en::*;
use fake::Fake;
use rand::Rng;
use std::cell::Cell;
use std::fmt;

use crate::determine;
use crate::entity;
use crate::Choice;
use crate::Outcome;

use block_id::{Alphabet, BlockId};
use rand::distributions::{Distribution, Standard};
use tabled::{Table, Tabled};

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
    tag: String,
    score: i32,
    memory: Memory,
    personality_type: Personality,
}

impl Entity {
    pub fn new(p: Personality, tag: String) -> Self {
        Self {
            name: FirstName().fake(),
            score: 0,
            memory: Memory::new(),
            personality_type: p,
            tag,
        }
    }

    pub fn new_player(p: Personality, t: String) -> Entity {
        Entity::new(p, t)
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_tag(&self) -> &str {
        self.tag.as_str()
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
pub trait Player: fmt::Display + std::clone::Clone {
    fn choose(&self) -> Choice;
    fn play(&self, other: &Self) -> (Outcome, Outcome);
    fn record_result(&mut self, o: Outcome);
    fn add_played_for_round(self, name: String);
    fn get_name(&self) -> &str;
}

impl Player for Entity {
    fn choose(&self) -> Choice {
        choose(&self.personality_type, &self.memory)
    }

    fn play(&self, other: &Self) -> (Outcome, Outcome) {
        determine(self.choose(), other.choose())
    }

    fn record_result(&mut self, o: Outcome) {
        self.score += o.as_i32();
    }

    fn add_played_for_round(mut self, name: String) {
        self.get_memory().get_curr_round().push(name)
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
    curr_round: Vec<String>,
}

impl Memory {
    fn new() -> Self {
        Self {
            opp_last_move: Choice::COOPERATE, // everyone starts nice
            last_move: Choice::COOPERATE,     // everyone starts nice
            betrayed: Cell::new(0),
            curr_round: Vec::new(),
        }
    }
    fn get_curr_round(&mut self) -> &mut Vec<String> {
        &mut self.curr_round
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

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
