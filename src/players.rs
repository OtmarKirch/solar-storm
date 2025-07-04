use crate::elements::*;

pub struct Player {
    name: String,
    hand: Vec<Ressource>,
    action_tokens: u32,
    position: usize,
}

impl Player {
    //constructor
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: Vec::new(),
            action_tokens: 0,
            position: 5
        }
    }

    //factory
    pub fn init(names: Vec<String>) -> Vec<Self> {
        names.into_iter().map(|name| Player::new(name)).collect()
    }

    pub fn add_ressource(&mut self, ressource: Ressource) {
        self.hand.push(ressource);
    }

    //getters
    pub fn hand(&self) -> &Vec<Ressource> {
        &self.hand
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn action_tokens(&self) -> u32 {
        self.action_tokens
    }

    pub fn position(&self) -> usize {
        self.position
    }

    //setters
    pub fn set_action_tokens(&mut self, gain: i32) -> u32 {
        if self.action_tokens as i32 + gain >= 0 {
        self.action_tokens = self.action_tokens + gain as u32;
        return self.action_tokens
        } else {
            panic!("Action tokens cannot be negative");
        }
    }

    pub fn add_ressources(&mut self, ressources: Vec<Ressource>) -> &Vec<Ressource> {
        self.hand.extend(ressources);
        &self.hand
    }

    pub fn remove_ressources(&mut self, ressources: Vec<Ressource>) -> &Vec<Ressource> {
        for ressource in ressources {
            if let Some(pos) = self.hand.iter().position(|x| *x == ressource) {
                self.hand.remove(pos);
            } else {
                panic!("Ressource not found in hand");
            }
        }
        &self.hand
    }

    pub fn move_position(&mut self, new_position: usize) -> usize {
        if new_position < 0 || new_position > 9 {
            panic!("Invalid position: {}", new_position);
        }
        match self.position {
            1 => {
                if !(new_position == 2 || new_position == 4) {
                    panic!("Invalid move from position 1 to {}", new_position);
                }
            },
            2 => {
                if !(new_position == 1 || new_position == 3 || new_position == 5) {
                    panic!("Invalid move from position 2 to {}", new_position);
                }
            },
            3 => {
                if !(new_position == 2 || new_position == 6) {
                    panic!("Invalid move from position 3 to {}", new_position);
                }
            },
            4 => {
                if !(new_position == 1 || new_position == 5 || new_position == 7) {
                    panic!("Invalid move from position 4 to {}", new_position);
                }
            },
            5 => {
                if !(new_position == 2 || new_position == 4 || new_position == 6 || new_position == 8) {
                    panic!("Invalid move from position 5 to {}", new_position);
                }
            },
            6 => {
                if !(new_position == 3 || new_position == 5 || new_position == 9) {
                    panic!("Invalid move from position 6 to {}", new_position);
                }
            },
            7 => {
                if !(new_position == 4 || new_position == 8) {
                    panic!("Invalid move from position 7 to {}", new_position);
                }
            },
            8 => {
                if !(new_position == 5 || new_position == 7 || new_position == 9) {
                    panic!("Invalid move from position 8 to {}", new_position);
                }
            },
            9 => {
                if !(new_position == 6 || new_position == 8) {
                    panic!("Invalid move from position 9 to {}", new_position);
                }
            },
            _ => panic!("Invalid current position: {}", self.position),

        }

        self.position = new_position;
        self.position
    }
}