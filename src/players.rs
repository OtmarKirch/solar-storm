use crate::elements::*;

pub struct Player {
    name: String,
    hand: Vec<Ressource>,
    action_tokens: u32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: Vec::new(),
            action_tokens: 0,
        }
    }

    pub fn add_ressource(&mut self, ressource: Ressource) {
        self.hand.push(ressource);
    }

    pub fn get_hand(&self) -> &Vec<Ressource> {
        &self.hand
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}