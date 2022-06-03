use crate::entity;

pub struct Game<E: entity::Player> {
    pub name: String,
    pub players: Vec<E>,
}

impl<E: entity::Player> Game<E> {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn add_player(&mut self, player: E) {
        self.players.push(player)
    }

    pub fn get_players(self) -> Vec<impl entity::Player> {
        self.players
    }
}

// fn build_group(players: &mut Vec<impl entity::Player>
// )
