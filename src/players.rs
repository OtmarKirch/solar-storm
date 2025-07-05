use crate::elements::*;

pub struct Player {
    name: String,
    hand: Vec<Ressource>,
    action_tokens: u32,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub enum PlayerActionError {
    InvalidDestination { current: usize, attempted: usize },
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

    //getters
    pub fn hand(&mut self) -> &mut Vec<Ressource> {
        &mut self.hand
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
    pub fn add_sub_action_tokens(&mut self, gain: i32) -> u32 {
        if self.action_tokens as i32 + gain >= 0 {
        self.action_tokens = (self.action_tokens as i32 + gain) as u32;
        return self.action_tokens
        } else {
            panic!("Action tokens cannot be negative");
        }
    }

    pub fn add_ressources(&mut self, ressources: Vec<Ressource>) -> &Vec<Ressource> {
        self.hand.extend(ressources);
        &self.hand
    }

    /* pub fn remove_ressources(&mut self, ressources: Vec<Ressource>) -> bool {
        for ressources in ressources.iter() {
            if !self.hand.contains(ressources) {
                return false; // If any ressource is not in hand, return false
            }
        }
        
        for ressource in ressources {
            if let Some(pos) = self.hand.iter().position(|x| *x == ressource) {
                self.hand.remove(pos);
                return true;
            } else {
                return false;
            }
        }
    } */

    pub fn move_position(&mut self, new_position: usize) -> Result<(), PlayerActionError> {
        if new_position > 9 {            
            panic!("Values above 9 should not be able to reach this method. Value is {}", new_position);
        }
        match self.position {
            1 => if !(new_position == 2 || new_position == 4) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })                   
                },
            2 => if !(new_position == 1 || new_position == 3 || new_position == 5) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })
                },
            3 => if !(new_position == 2 || new_position == 6) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })  
                },
            4 => if !(new_position == 1 || new_position == 5 || new_position == 7) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })  
                },
            5 => if !(new_position == 2 || new_position == 4 || new_position == 6 || new_position == 8) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })  
                },
            6 => if !(new_position == 3 || new_position == 5 || new_position == 9) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })  
                },
            7 => if !(new_position == 4 || new_position == 8) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })  
                },
            8 => if !(new_position == 5 || new_position == 7 || new_position == 9) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })  
                },
            9 => if !(new_position == 6 || new_position == 8) {
                    return Err(PlayerActionError::InvalidDestination {
                        current: self.position,
                        attempted: new_position,
                    })  
                },
            _ => panic!("Invalid current position: {}", self.position),

        }

        self.position = new_position;
        Ok(())
    }

    pub fn set_position(&mut self, new_position: usize) -> usize {
        if new_position > 9 {
            panic!("Invalid position: {}", new_position);
        }
        self.position = new_position;
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let mut player = Player::new("Alice".to_string());
        assert_eq!(player.name(), "Alice");
        assert_eq!(player.hand().len(), 0);
        assert_eq!(player.action_tokens(), 0);
        assert_eq!(player.position(), 5);
    }

    #[test]
    fn test_player_init() {
        let names = vec!["Alice".to_string(), "Bob".to_string()];
        let players = Player::init(names);
        assert_eq!(players.len(), 2);
        assert_eq!(players[0].name(), "Alice");
        assert_eq!(players[1].name(), "Bob");        
    }

    #[test]
    fn test_add_sub_action_tokens() {
        let mut player = Player::new("Alice".to_string());
        assert_eq!(player.add_sub_action_tokens(3), 3);
        assert_eq!(player.action_tokens(), 3);
        assert_eq!(player.add_sub_action_tokens(-1), 2);
        assert_eq!(player.action_tokens(), 2);
    }

    #[test]
    #[should_panic(expected = "Action tokens cannot be negative")]
    fn test_add_sub_action_tokens_negative() {
        let mut player = Player::new("Alice".to_string());
        player.add_sub_action_tokens(-1);
    }

    #[test]
    fn test_add_ressources() {
        let mut player = Player::new("Alice".to_string());
        player.add_ressources(vec![Ressource::new(RessourceType::Data), Ressource::new(RessourceType::Metal)]);
        player.add_ressources(vec![Ressource::new(RessourceType::Energy), Ressource::new(RessourceType::Nanobots)]);
        assert_eq!(player.hand().len(), 4);
        assert_eq!(player.hand()[0].ressource_type(), &RessourceType::Data);
        assert_eq!(player.hand()[1].ressource_type(), &RessourceType::Metal);
        assert_eq!(player.hand()[2].ressource_type(), &RessourceType::Energy);
        assert_eq!(player.hand()[3].ressource_type(), &RessourceType::Nanobots);
    }
    /* #[test]
    fn test_remove_ressources() {
        let mut player = Player::new("Alice".to_string());
        player.add_ressources(vec![Ressource::new(RessourceType::Data), Ressource::new(RessourceType::Metal)]);
        player.add_ressources(vec![Ressource::new(RessourceType::Energy), Ressource::new(RessourceType::Nanobots)]);      
        assert_eq!(player.hand().len(), 4);

        player.remove_ressources(vec![Ressource::new(RessourceType::Data), Ressource::new(RessourceType::Energy)]);
        assert_eq!(player.hand().len(), 2);
        assert_eq!(player.hand()[0].ressource_type(), &RessourceType::Metal);
        assert_eq!(player.hand()[1].ressource_type(), &RessourceType::Nanobots);
    } */

    #[test]
    fn test_move_position() {
        let mut player = Player::new("Alice".to_string());
        assert_eq!(player.move_position(2), Ok(()));
        assert_eq!(player.position(), 2);
    }

}