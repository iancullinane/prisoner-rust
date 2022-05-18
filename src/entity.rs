use fake::faker::name::en::*;
use fake::Fake;
use rand::Rng;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;

use crate::Choice;
use crate::Outcome;
// A concrete struct from which to base the Player trait
#[derive(Clone, Debug)]
pub struct Entity {
    // TODO::Generate short id's https://github.com/drifting-in-space/block-id
    name: String,
    score: Cell<i16>,
    memory: Memory,
}

impl Entity {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_score(&self) -> i16 {
        self.score.get()
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn record_result(&self, o: Outcome) {
        self.score.set(self.score.get() + o.as_i16());
    }
}

// Player is the trait to repesent a player of the game
// most notably the players behavior implementation
pub trait Player {
    fn get_name(&self) -> String {
        self.get_entity().get_name()
    }
    fn get_behavior(&self) -> String;
    fn get_entity(&self) -> &Entity; // TODO::this and line 38 are coupled, figure out how to do it better
    fn choose(&self) -> Choice;
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

// The following are implementations of three personalities in the game

#[derive(Clone, Debug)]
pub struct AlwaysCooperate {
    entity: Entity,
}

impl Player for AlwaysCooperate {
    fn get_behavior(&self) -> String {
        String::from("Always Cooperate")
    }
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn choose(&self) -> Choice {
        Choice::COOPERATE
    }
}

pub struct AlwaysCheat {
    entity: Entity,
}

impl Player for AlwaysCheat {
    fn get_behavior(&self) -> String {
        String::from("Always Cheat")
    }
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn choose(&self) -> Choice {
        Choice::CHEAT
    }
}

pub struct CopyCat {
    entity: Entity,
}

impl Player for CopyCat {
    fn get_behavior(&self) -> String {
        String::from("Copycat")
    }
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn choose(&self) -> Choice {
        let mem = self.get_entity().get_memory();
        mem.opp_last_move.clone()
    }
}

pub struct SlowLearner {
    entity: Entity,
}

impl Player for SlowLearner {
    fn get_behavior(&self) -> String {
        String::from("Slow learner")
    }
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn choose(&self) -> Choice {
        let mem = self.get_entity().get_memory();
        if mem.betrayed.get() > 10 {
            return Choice::CHEAT;
        }
        Choice::COOPERATE
    }
}

pub struct Contrarian {
    entity: Entity,
}

impl Player for Contrarian {
    fn get_behavior(&self) -> String {
        String::from("Contrarian")
    }
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn choose(&self) -> Choice {
        let mem = self.get_entity().get_memory();
        if mem.last_move == mem.opp_last_move {
            return reverse(mem.last_move);
        }
        mem.last_move
    }
}

// PlayerFacory returns a random player
pub struct PlayerFactory;
impl PlayerFactory {
    pub fn get_player() -> Box<dyn Player> {
        let random_number: u32 = rand::thread_rng().gen_range(1..101);
        match random_number {
            0..=10 => Box::new(AlwaysCooperate {
                entity: Entity {
                    name: FirstName().fake(),
                    score: Cell::new(0),
                    memory: Memory::new(),
                },
            }),
            11..=19 => Box::new(Contrarian {
                entity: Entity {
                    name: FirstName().fake(),
                    score: Cell::new(0),
                    memory: Memory::new(),
                },
            }),
            20..=70 => Box::new(SlowLearner {
                entity: Entity {
                    name: FirstName().fake(),
                    score: Cell::new(0),
                    memory: Memory::new(),
                },
            }),
            71..=89 => Box::new(AlwaysCheat {
                entity: Entity {
                    name: FirstName().fake(),
                    score: Cell::new(0),
                    memory: Memory::new(),
                },
            }),
            _ => Box::new(CopyCat {
                entity: Entity {
                    name: FirstName().fake(),
                    score: Cell::new(0),
                    memory: Memory::new(),
                },
            }),
        }
    }
}

// For printing outcomes
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!("\t{}\t{}", self.get_name(), self.get_score());
        write!(f, "{}", output)
    }
}

impl fmt::Display for dyn Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create the table
        let e = self.get_entity();
        let output = format!(
            "[{}]\t{}\t{}",
            self.get_behavior(),
            e.get_name(),
            e.get_score()
        );
        write!(f, "{}", output)
    }
}
