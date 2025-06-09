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
use std::fmt;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use tabled::Tabled;

use crate::determine;
use crate::memory::{InMemoryStore, Meme, MemoryStore};
use crate::Choice;
use crate::Outcome;

/// Personality is an enum used to select an entities behavior. The class function
/// `choose` holds logic depending on which personality is being used. Some
// personalities requires access to memory.
#[derive(Clone, Copy, Debug, EnumCountMacro, EnumIter)]
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
        match rng.gen_range(0..Personality::COUNT) {
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
    memory: Box<dyn MemoryStore>,
    personality_type: Personality,
}

impl Entity {
    pub fn new(personality_type: Personality, tag: String) -> Self {
        Self {
            name: FirstName().fake(),
            score: 0,
            memory: Box::new(InMemoryStore::new()),
            personality_type,
            tag,
        }
    }

    pub fn full_info(&self) -> String {
        format!(
            "Name: {}\tScore: {}\tPersonality: {:?}",
            self.name, self.score, self.personality_type
        )
    }

    pub fn get_memory(&self) -> &dyn MemoryStore {
        &self.memory
    }
}

// Choose parses all personalities
fn choose(p: &Personality, m: &dyn MemoryStore, _opp_tag: &str) -> Choice {
    match p {
        Personality::AlwaysCooperate => Choice::COOPERATE,
        Personality::AlwaysCheat => Choice::CHEAT,
        Personality::CopyCat => m.last_move(),
        Personality::Vengeful => {
            if m.betrayed_count() > 1 {
                // println!("Vengeful cheats");
                return Choice::CHEAT;
            }
            Choice::COOPERATE
        }
        Personality::SlowLearner => {
            if m.betrayed_count() > 5 {
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
    fn memory(&self) -> &dyn MemoryStore;
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
            .history()
            .iter()
            .map(|m| {
                let (o1, _) = determine(m.player_choice, m.opp_choice);
                Outcome::positive(&o1)
            })
            .for_each(|num| new_sum += num);
        self.score = new_sum;
        new_sum
    }

    // add_memory adds a new memory, updates the top level fields and updates the score
    fn add_memory(&mut self, tag: &str, choices: (Choice, Choice)) {
        self.memory.add(tag, choices);
        self.score();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn memory(&self) -> &dyn MemoryStore {
        &*self.memory
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
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
        write!(
            f,
            "Player Choice: {}, Opponent () Choice: {}",
            self.player_choice,
            // find_name_from_tag(self.opp_tag),
            self.opp_choice
        )
    }
}


// pub fn find<'a, T, F>(f: F, players: &'a [T]) -> Option<&'a T>
// where
//     T: Player,
//     F: Fn(&&T) -> bool,
// {
//     players.iter().find(f)
// }

// pub fn find_mut<'a, T, F>(f: F, players: &'a mut [T]) -> Option<&'a mut T>
// where
//     T: Player,
//     F: Fn(&&mut T) -> bool,
// {
//     players.iter_mut().find(f)
// }

pub fn find<'a>(tag: &str, players: &'a [impl Player]) -> Option<&'a impl Player> {
    players.iter().find(|&player| player.tag() == tag)
}

pub fn find_mut<'a>(tag: &str, players: &'a mut [impl Player]) -> Option<&'a mut impl Player> {
    players.iter_mut().find(|player| player.tag() == tag)
}

// // find_by_player takes a player name and a slice of players and returns the player
// pub fn find_by_name(name: &str, players: &[Entity]) -> Option<Entity> {
//     players.iter().find(|player| player.name() == name).cloned()
// }

// // find_by_player takes a player name and a slice of players and returns the player
// pub fn find_name_from_tag(tag: &str, players: &[Entity]) -> String {
//     let player = players.iter().find(|player| player.tag() == tag).cloned();
//     match player {
//         Some(player) => player.name().to_string(),
//         None => "Name not found".to_string(),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_move_tracks_latest() {
        let mut e = Entity::new(Personality::CopyCat, "a".to_string());

        // First interaction: opponent cheats
        e.add_memory("b", (Choice::COOPERATE, Choice::CHEAT));
        assert_eq!(e.memory().last_move(), Choice::CHEAT);

        // Second interaction: opponent cooperates
        e.add_memory("c", (Choice::COOPERATE, Choice::COOPERATE));
        assert_eq!(e.memory().last_move(), Choice::COOPERATE);

        // CopyCat should mirror the last move
        assert_eq!(e.choose("c"), Choice::COOPERATE);
    }
}
