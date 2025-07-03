/// This file contains the definitions of the elements used in the game Solar Storm.
///
///
/// 
/// 



use std::collections::HashSet;
use rand::prelude::*;

use crate::elements::ressources::*;


/// Rooms
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    room_type: RoomType,
    repair_field: [(RessourceType, bool); 3], // RessourceType and amount needed to repair
    divert_field: [RessourceType; 3],
    diverted: bool, 
    ability: String,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Version {
    V1,
    V2,
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
        }
    }
            

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
    rooms
    }

}

#[cfg(test)]
mod tests {
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
        assert!(room_types.contains(&RoomType::EnergyCore));      
    }
}