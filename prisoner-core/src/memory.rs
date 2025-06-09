use std::cell::Cell;

use crate::Choice;

#[derive(Clone, Debug)]
pub struct Meme {
    pub opp_tag: String,
    pub player_choice: Choice,
    pub opp_choice: Choice,
}

pub trait MemoryStore {
    fn add(&mut self, tag: &str, choices: (Choice, Choice));
    fn last_move(&self) -> Choice;
    fn history(&self) -> &[Meme];
    fn betrayed_count(&self) -> i32;
}

#[derive(Clone, Debug)]
pub struct InMemoryStore {
    history: Vec<Meme>,
    opp_last_move: Cell<Choice>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self { history: Vec::new(), opp_last_move: Cell::new(Choice::COOPERATE) }
    }
}

impl MemoryStore for InMemoryStore {
    fn add(&mut self, tag: &str, choices: (Choice, Choice)) {
        self.opp_last_move.replace(choices.1);
        self.history.insert(
            0,
            Meme { opp_tag: tag.to_string(), player_choice: choices.0, opp_choice: choices.1 },
        );
    }

    fn last_move(&self) -> Choice {
        self.opp_last_move.get()
    }

    fn history(&self) -> &[Meme] {
        &self.history
    }

    fn betrayed_count(&self) -> i32 {
        self.history.iter().filter(|m| m.opp_choice == Choice::CHEAT).count() as i32
    }
}

