/// This file contains the definitions of the elements used in the game Solar Storm.
///
///
/// 
/// 



use std::collections::HashSet;
use rand::prelude::*;

use crate::elements::ressources::*;
use crate::game::*;


/// Rooms
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    room_type: RoomType,
    repair_field: [(RessourceType, bool); 3], // RessourceType and amount needed to repair
    divert_field: [RessourceType; 3],
    diverted: bool, 
    ability: String,
    protected: Vec<String>, // Indicates if the room is protected
    version: Version,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoomType {
    CrewQuarters,
    EngineRoom,
    Armoury,
    MedicalBay,
    CargoHold,
    RepairCenter,
    MessHall,
    Bridge,
    EnergyCore,
    AnyRoom
}


impl Room {
    fn new(
        room_type: RoomType,
        repair_field: [(RessourceType, bool); 3],
        divert_field: [RessourceType; 3],
        ability: String,
        version: Version,
    ) -> Self {
        Self {
            room_type,
            repair_field,
            divert_field,
            ability,
            version,
            diverted: false,
            protected: vec![],
        }
    }
            
    /// All 9 rooms are initialized here. A set of rooms is created based on the version of the game. The order of the rooms is randomized, while the EnergyCore is always the fifth element in the vector.
    /// The placement of the room in the game is determined by the order of the rooms in the vector. Room 1-3 are placed in the first row, room 4-6 in the second row, and room 7-9 in the third row. This determines how players can move their meeples to the rooms.
    pub fn init_rooms(version: Version) -> Vec<Room> {
        let mut rooms = Vec::new();

        // Define the rooms based on the version
        match version {
            Version::V1 => {
                rooms.push(
                    Room::new(
                        RoomType::CrewQuarters,
                        [
                                (RessourceType::Energy, false),
                                (RessourceType::Data, false),
                                (RessourceType::Metal, false),
                            ],
                            [
                                RessourceType::Metal,
                                RessourceType::Metal,
                                RessourceType::Nanobots,
                            ],
                        
                        "Move a player's meeple to a room that has another meeple in it"
                                    .to_string(),
                        Version::V1,
                        
                    )
                    
                );
                rooms.push(
                    Room::new(
                        RoomType::EngineRoom,
                        [
                                (RessourceType::Energy, false),
                                (RessourceType::Nanobots, false),
                                (RessourceType::Data, false),
                            ],
                        [
                                RessourceType::Energy,
                                RessourceType::Metal,
                                RessourceType::Nanobots,
                            ],                       
                        "Move a player's meeple to a room that has another meeple in it"
                                    .to_string(),
                        Version::V1,
                        
                    )
                );
                rooms.push(
                    Room::new(
                        RoomType::Armoury,
                        [
                                (RessourceType::Data, false),
                                (RessourceType::Nanobots, false),
                                (RessourceType::Metal, false),
                            ],
                        [
                                RessourceType::Metal,
                                RessourceType::Energy,
                                RessourceType::Nanobots,
                            ],                       
                        "Place 2 protection tokens on any room[s] [this ends at the start of your next turn]"
                                    .to_string(),
                        Version::V1,
                        
                    )
                );
                rooms.push(Room::new(
                        RoomType::MedicalBay,
                        [
                                (RessourceType::Metal, false),
                                (RessourceType::Nanobots, false),
                                (RessourceType::Energy, false),
                            ],
                        [
                                RessourceType::Data,
                                RessourceType::Energy,
                                RessourceType::Energy,
                            ],                       
                        "As two actions, give three action tokens to any other player[s]"
                                    .to_string(),
                        Version::V1,
                        
                    )
                );
                rooms.push(
                    Room::new(
                        RoomType::CargoHold,
                        [
                                (RessourceType::Energy, false),
                                (RessourceType::Metal, false),
                                (RessourceType::Data, false),
                            ],
                        [
                                RessourceType::Data,
                                RessourceType::Metal,
                                RessourceType::Nanobots,
                            ],                       
                        "Look at the next 5 resource cards. Then put them back in any order."
                                    .to_string(),
                        Version::V1,
                        
                    )
                );
                rooms.push(
                    Room::new(
                        RoomType::RepairCenter,
                        [
                                (RessourceType::Metal, false),
                                (RessourceType::Energy, false),
                                (RessourceType::Nanobots, false),
                            ],
                        [
                                RessourceType::Data,
                                RessourceType::Energy,
                                RessourceType::Nanobots,
                            ],                       
                        "Repair a damaged room by one space on the Repair Track. Discard the matching card."
                                    .to_string(),
                        Version::V1,
                        
                    )
                );
                rooms.push(
                    Room::new(
                        RoomType::MessHall,
                        [
                                (RessourceType::Nanobots, false),
                                (RessourceType::Energy, false),
                                (RessourceType::Data, false),
                            ],
                        [
                                RessourceType::Data,
                                RessourceType::Data,
                                RessourceType::Energy,
                            ],                       
                        "Give, take, or exchange a resource card with another player."
                                    .to_string(),
                        Version::V1,
                        
                    )
                );
                rooms.push(
                    Room::new(
                        RoomType::Bridge,
                        [
                                (RessourceType::Nanobots, false),
                                (RessourceType::Data, false),
                                (RessourceType::Energy, false),
                            ],
                        [
                                RessourceType::Data,
                                RessourceType::Metal,
                                RessourceType::Energy,
                            ],                       
                        "Look at the next 3 Damage cards and put them back in any order."
                                    .to_string(),
                        Version::V1,
                    )
                );
            }
            Version::V2 => {
                rooms.push(
                    Room::new(
                        RoomType::EnergyCore,
                        [
                                (RessourceType::Universal, false),
                                (RessourceType::Universal, false),
                                (RessourceType::Universal, false),
                            ],
                        [
                                RessourceType::Universal,
                                RessourceType::Universal,
                                RessourceType::Universal,
                            ],                       
                        "When all rooms have diverted power, get here and use 1 action to reactivate the Energy Core."
                                    .to_string(),
                        Version::V2,
                        
                    )
                );
            }
        }
        // Shuffle the rooms
        let mut rng = rand::rng();
        rooms.shuffle(&mut rng);

        rooms.insert(4,
                    Room::new(
                        RoomType::EnergyCore,
                        [
                                (RessourceType::Universal, false),
                                (RessourceType::Universal, false),
                                (RessourceType::Universal, false),
                            ],
                        [
                                RessourceType::Universal,
                                RessourceType::Universal,
                                RessourceType::Universal,
                            ],                       
                        "When all rooms have diverted power, get here and use 1 action to reactivate the Energy Core."
                                    .to_string(),
                        Version::V1,
                    )
                );

        rooms
    }

    //getters
    pub fn room_type(&self) -> &RoomType {
        &self.room_type
    }

    pub fn repair_field(&mut self) -> &mut [(RessourceType, bool); 3] {
        &mut self.repair_field
    }

    pub fn is_protected(&self) -> bool {
        !self.protected.is_empty()
    }

    //setters
    pub fn set_shield(&mut self, player_name: String) {
        self.protected.push(player_name);
    }

    /// This method damages the first field in the repair field that has not been damaged yet, and return true if a field was successfully damaged. 
    /// If all fields are already damaged, it returns false. This can be used to determine if the end game condition has been reached.
    pub fn damage_room(&mut self) -> bool {
        let repair_field = &mut self.repair_field;
        let first_healthy_field = dbg!(repair_field.iter_mut().find(|(_, damaged)| damaged == &false));
        if let Some((_, damaged)) = first_healthy_field {
            *damaged = true; // Set the first healthy field to damaged
            true
        } else {
            false // All repair fields are already damaged. This bool can be used to determine that end game condition has been reached.
        }
    }


    /// This method implements the removal of the protection tokens at the start of the player's turn.
    pub fn check_unset_shield(rooms: &mut Vec<Room>, player_name: String) {
        for room in rooms.iter_mut() {
            if let Some(pos) = room.protected.iter().position(|x| *x == player_name) {
                room.protected.remove(pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::rooms;

    use super::*;

    #[test]
    fn test_init_rooms() {
        let rooms = dbg!(Room::init_rooms(Version::V1));
        assert_eq!(rooms.len(), 9, "Expected 9 rooms");
        let room_types: HashSet<RoomType> = rooms.iter().map(|r| r.room_type.clone()).collect();
        assert!(room_types.contains(&RoomType::CrewQuarters));
        assert!(room_types.contains(&RoomType::EngineRoom));
        assert!(room_types.contains(&RoomType::Armoury));
        assert!(room_types.contains(&RoomType::MedicalBay));
        assert!(room_types.contains(&RoomType::CargoHold));
        assert!(room_types.contains(&RoomType::RepairCenter));
        assert!(room_types.contains(&RoomType::MessHall));
        assert!(room_types.contains(&RoomType::Bridge));
        assert_eq!(rooms[4].room_type, RoomType::EnergyCore);      
    }

    #[test]
    fn test_room_type() {
        let room = Room::new(
            RoomType::CrewQuarters,
            [
                (RessourceType::Energy, false),
                (RessourceType::Data, false),
                (RessourceType::Metal, false),
            ],
            [
                RessourceType::Metal,
                RessourceType::Metal,
                RessourceType::Nanobots,
            ],
            "Move a player's meeple to a room that has another meeple in it".to_string(),
            Version::V1,
        );
        assert_eq!(room.room_type(), &RoomType::CrewQuarters);
    }

    #[test]
    fn test_repair_field() {
        let mut rooms = Room::init_rooms(Version::V1);
        let mut room = &mut rooms[0];
        let room_type = &room.room_type;
        let mut repair_field = room.repair_field();

        let repair_field_hash: HashSet<_> = repair_field.iter().map(|(r_type, _)| r_type).collect();
        assert_eq!(repair_field_hash.len(), 3, "Expected repair field to have 3 unique RessourceTypes");

        for field in repair_field.iter() {
            assert_eq!(field.1, false, "Expected all repair fields to have a boolean value of false after initialization");
        }

        repair_field[1].1 = true; // Simulate repairing the second field
        assert_eq!(repair_field[1].1, true, "Expected the second repair field to be set to true after damage");
    }

    #[test]
    fn test_damage_room() {
        let mut rooms = Room::init_rooms(Version::V1);
        let mut room = &mut rooms[0];
        let repair_field = room.repair_field();
        room.damage_room();
        let repair_field = dbg!(room.repair_field());
        assert_eq!(repair_field[0].1, true, "Expected the first repair field to be set to true after damage");
    }

    #[test]
    fn test_damage_room_all_fields_damaged() {
        let mut rooms = Room::init_rooms(Version::V1);
        let room = &mut rooms[0];
        let repair_field = room.repair_field();
        // Set all repair fields to damaged
        for field in repair_field.iter_mut() {
            field.1 = true;
        }
        assert!(!room.damage_room(), "Expected damage_room to return false when all fields are already damaged");
    }
}