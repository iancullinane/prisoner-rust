//! Entity is a concrete base type and Player implementation
//!
//! Entity provides a memory structure which keeps track or who this entity
//! has played in its lifetime, and the outcomes. Names are generated as on
//! creation of a new entity.
//!

use fake::faker::name::en::*;
use fake::Fake;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::cell::Cell;
use std::fmt;
use tabled::Tabled;

use crate::determine;
use crate::Choice;
use crate::Outcome;

/// Personality is an enum used to select an entities behavior. The class function
/// `choose` holds logic depending on which personality is being used. Some
// personalities requires access to memory.
#[derive(Clone, Copy, Debug)]
pub enum Personality {
    /// Will always choose COOPERATE
    AlwaysCooperate,
    /// Will always choose CHEAT
    AlwaysCheat,
    /// Will do the move its opponenet did the last round
    CopyCat,
    /// Once betrayed, will always CHEAT
    Vengeful,
    /// Will only CHEAT if betrayed more than 5 times
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

/// A concrete base class to hold game data.
#[derive(Clone, Debug, Tabled)]
pub struct Entity {
    name: String,
    #[tabled(skip)]
    tag: String,
    score: i32,
    #[tabled(skip)]
    memory: Memory,
    personality_type: Personality,
}

impl Entity {
    pub fn new(personality_type: Personality, tag: String) -> Self {
        Self {
            name: FirstName().fake(),
            score: 0,
            memory: Memory::new(),
            personality_type,
            tag,
        }
    }
}

// Choose parses all personalities
fn choose(p: &Personality, m: &Memory, _opp_tag: &str) -> Choice {
    match p {
        Personality::AlwaysCooperate => Choice::COOPERATE,
        Personality::AlwaysCheat => Choice::CHEAT,
        Personality::CopyCat => m.opp_last_move.get(),
        Personality::Vengeful => {
            if m.betrayed() > 1 {
                // println!("Vengeful cheats");
                return Choice::CHEAT;
            }
            Choice::COOPERATE
        }
        Personality::SlowLearner => {
            if m.betrayed() > 5 {
                // println!("Slow learner cheats");
                return Choice::CHEAT;
            }
            Choice::COOPERATE
        }
    }
}

/// Any concrete type which provides the methods required to play the The
/// Prisoner's Dilemna, and information for display.
pub trait Player: fmt::Display + std::clone::Clone {
    fn choose(&self, opp_tag: &str) -> Choice;
    fn name(&self) -> &str;
    fn memory(&self) -> &Memory;
    fn score(&mut self) -> i32;
    fn tag(&self) -> &str;
    fn add_memory(&mut self, tag: &str, other: (Choice, Choice));
}

impl Player for Entity {
    fn choose(&self, opp_tag: &str) -> Choice {
        choose(&self.personality_type, &self.memory, opp_tag)
    }

    // score derives the players current score from the move history
    fn score(&mut self) -> i32 {
        let mut new_sum: i32 = 0;
        // let last = self.memory.opp_last_move;
        self.memory
            .history
            .iter()
            .map(|m| {
                let (o1, _) = determine(m.player_choice, m.opp_choice);
                self.memory.opp_last_move.replace(m.opp_choice);
                Outcome::positive(&o1)
            })
            .for_each(|num| new_sum += num);
        self.score = new_sum;
        new_sum
    }

    // add_memory adds a new memory, updates the top level fields and updates the score
    fn add_memory(&mut self, tag: &str, choices: (Choice, Choice)) {
        self.memory.opp_last_move.replace(choices.1);
        self.memory
            .history
            .insert(0, Meme::new(tag.to_string(), choices.0, choices.1));
        self.score();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn memory(&self) -> &Memory {
        &self.memory
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
    }
}

/// A list of moves and information about the last move
#[derive(Clone, Debug)]
pub struct Memory {
    history: Vec<Meme>,
    opp_last_move: Cell<Choice>,
}

/// Represents an interactoin with another player
#[derive(Clone, Debug)]
pub struct Meme {
    // opp_tag: String,
    player_choice: Choice,
    opp_choice: Choice,
}

impl Meme {
    fn new(_opp_tag: String, p_choice: Choice, opp_choice: Choice) -> Self {
        Self {
            // opp_tag: tag,
            player_choice: p_choice,
            opp_choice,
        }
    }
}

impl Memory {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            opp_last_move: Cell::new(Choice::COOPERATE), // everyone starts nice
        }
    }

    // total betrayals
    fn betrayed(&self) -> i32 {
        self.history
            .iter()
            .filter(|x| x.opp_choice == Choice::CHEAT)
            .fold(0, |acc, _| acc + 1)
    }
}

// For printing outcomes
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self.name.to_string();
        // let output = format!(
        //     "{}\t{}\t{}",
        //     self.name(),
        //     self.get_score(),
        //     self.get_personality_type()
        // );
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

impl fmt::Display for Meme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let memes: i32 = self.history.iter().len() as i32;
        write! {f, "{}", memes}
    }
}
