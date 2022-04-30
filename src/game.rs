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

    // pub fn get_players(&self) -> Vec<entity::Entity> {
    //     self.players.
    // }

    // fn get_outcome(&self) -> Outcome {
    //     let random_number = rand::thread_rng().gen_range(1..101);
    //     match random_number.cmp(&MIDDLE) {
    //         Ordering::Less => Outcome::COOOPERATE,
    //         Ordering::Greater => Outcome::CHEAT,
    //         Ordering::Equal => self.get_outcome(),
    //     }
    // }
}
