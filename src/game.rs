use crate::entity;

pub struct Game {
    pub name: String,
    pub players: Vec<Box<dyn entity::Player>>,
}

impl Game {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_players(&mut self) -> &mut Vec<Box<dyn entity::Player>> {
        &mut self.players
    }


}
