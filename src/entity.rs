use rand::Rng;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;

use crate::Choice;
use crate::Outcome;
use crate::{determine, once};
use fake::faker::name::en::*;
use fake::Fake;

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
            if m.betrayed() > 1 {
                return Choice::CHEAT;
            }
            Choice::COOPERATE
        }
        Personality::SlowLearner => {
            if m.betrayed() > 5 {
                return Choice::CHEAT;
            }
            Choice::COOPERATE
        }
    }
}

// Player is the trait to repesent a player of the game
// most notably the players behavior implementation
pub trait Player: fmt::Display + std::clone::Clone {
    fn choose(&self, opp_tag: &str) -> Choice;
    fn play(&mut self, other: &mut Self);
    fn name(&self) -> &str;
    fn score_outcome(&mut self, o: &Outcome);
    fn score(&self) -> i32;
    fn tag(&self) -> &str;
    fn add_memory(&mut self, tag: &str, other: &Outcome);
}

impl Player for Entity {
    fn choose(&self, opp_tag: &str) -> Choice {
        choose(&self.personality_type, &self.memory)
    }

    fn play(&mut self, other: &mut Self) {
        let (o1, o2) = determine(self.choose(other.tag()), other.choose(self.tag()));
        self.score_outcome(&o1);
        other.score_outcome(&o2);
        self.add_memory(other.tag(), &o2);
        other.add_memory(self.tag(), &o1);
    }

    fn score(&self) -> i32 {
        self.score
    }

    fn add_memory(&mut self, tag: &str, other: &Outcome) {
        let m = self.get_memory();
    }

    fn score_outcome(&mut self, o: &Outcome) {
        self.score += Outcome::positive_scoring(o) as i32;
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
    }
}

// Memory is used when making a choice on how to play
// TODO::implement a lot more functions on memory
#[derive(Clone, Debug)]
pub struct Memory {
    opp_last_move: Choice,
    last_move: Choice,
    moves: Vec<Meme>,
}

#[derive(Clone, Debug)]
struct Meme {
    oppTag: String,
    outcomes: Vec<Outcome>,
}

impl Memory {
    fn new() -> Self {
        Self {
            opp_last_move: Choice::COOPERATE, // everyone starts nice
            last_move: Choice::COOPERATE,
            moves: Vec::new(),
        }
    }

    fn betrayed(&self) -> i32 {
        // self.outcomes.iter();
        1
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
            self.name(),
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

// Example of newtype pattern
// struct PlayerList(pub Vec<Entity>);
// impl fmt::Display for PlayerList {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

//         // self.0.iter().fold(Ok(()), |result, album| {
//         //     result.and_then(|_| writeln!(f, "{}", album))
//         // })
//     }
// }

// pub fn print_result(players: &Vec<Box<dyn entity::Player>>) {
//     let mut table = Table::new();

//     // Add a row per time
//     table.add_row(row!["Behavior", "Name", "Score"]);
//     for p in players {
//         table.add_row(Row::new(vec![
//             tCell::new(&p.get_behavior()),
//             tCell::new(&p.get_name()),
//             tCell::new(&p.get_entity().get_score().to_string()),
//         ]));
//     }

//     let format = format::FormatBuilder::new()
//         .column_separator('|')
//         .borders('|')
//         .separators(
//             &[format::LinePosition::Top, format::LinePosition::Bottom],
//             format::LineSeparator::new('-', '+', '+', '+'),
//         )
//         .padding(1, 1)
//         .build();
//     table.set_format(format);
//     table.printstd();
// }
