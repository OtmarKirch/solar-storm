/// This file contains the definitions of the elements used in the game Solar Storm.
///
///
/// 
/// 



use std::collections::HashSet;

use crate::elements::ressources::*;


/// Rooms


#[derive(Debug)]
pub enum Room {
    CrewQuarters(Version, RepairField, DivertField, Ability),
    EngineRoom(Version, RepairField, DivertField, Ability),
    Armoury(Version, RepairField, DivertField, Ability),
    MedicalBay(Version, RepairField, DivertField, Ability),
    CargoHold(Version, RepairField, DivertField, Ability),
    RepairCenter(Version, RepairField, DivertField, Ability),
    MessHall(Version, RepairField, DivertField, Ability),
    Bridge(Version, RepairField, DivertField, Ability),
    EnergyCore(Version, Ability),
}

impl Room {
    pub fn init_rooms(version: Version) -> Vec<Room> {
        let mut rooms = Vec::new();

        // Define the rooms based on the version
        match version {
            Version::V1 => {
                rooms.push(
                    Room::CrewQuarters(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Energy, false),
                                (RessourceType::Data, false),
                                (RessourceType::Metal, false),
                            ],
                        },
                        DivertField {
                            RessourceTypes: vec![
                                (RessourceType::Metal, false),
                                (RessourceType::Metal, false),
                                (RessourceType::Nanobots, false),
                            ],
                        },
                        Ability {
                            description:
                                "Move a player's meeple to a room that has another meeple in it"
                                    .to_string(),
                        },
                    ),
                );
                rooms.push(Room::EngineRoom(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Data, false),
                                (RessourceType::Metal, false),
                                (RessourceType::Nanobots, false),
                            ],
                        },
                        DivertField {
                            RessourceTypes: vec![
                                (RessourceType::Metal, false),
                                (RessourceType::Nanobots, false),
                                (RessourceType::Nanobots, false),
                            ],
                        },
                        Ability {
                            description: "Swap a card from your hand with one from the discard pile."
                                .to_string(),
                        },
                    ),
                );
                rooms.push(Room::Armoury(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Data, false),
                                (RessourceType::Nanobots, false),
                                (RessourceType::Metal, false),
                            ],
                        },
                        DivertField {
                            RessourceTypes: vec![
                                (RessourceType::Metal, false),
                                (RessourceType::Energy, false),
                                (RessourceType::Nanobots, false),
                            ],
                        },
                        Ability {
                            description:
                                "Place 2 protection tokens on any room[s] [this ends at the start of your next turn]"
                                    .to_string(),
                        },
                    ),
                );
                rooms.push(Room::MedicalBay(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Metal, false),
                                (RessourceType::Nanobots, false),
                                (RessourceType::Energy, false),
                            ],
                        },
                        DivertField {
                            RessourceTypes: vec![
                                (RessourceType::Data, false),
                                (RessourceType::Energy, false),
                                (RessourceType::Energy, false),
                            ],
                        },
                        Ability {
                            description:
                                "As two actions, give three action tokens to any other player[s]"
                                    .to_string(),
                        },
                    ),
                );
                rooms.push(Room::CargoHold(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Energy, false),
                                (RessourceType::Metal, false),
                                (RessourceType::Data, false),
                            ],
                        },
                        DivertField {
                            RessourceTypes: vec![
                                (RessourceType::Data, false),
                                (RessourceType::Metal, false),
                                (RessourceType::Nanobots, false),
                            ],
                        },
                        Ability {
                            description:
                                "Look at the next 5 resource cards. Then put them back in any order."
                                    .to_string(),
                        },
                    ),
                );
                rooms.push(Room::RepairCenter(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Metal, false),
                                (RessourceType::Energy, false),
                                (RessourceType::Nanobots, false),
                            ],
                        },
                        DivertField {
                            RessourceTypes: vec![
                                (RessourceType::Data, false),
                                (RessourceType::Energy, false),
                                (RessourceType::Nanobots, false),
                            ],
                        },
                        Ability {
                            description:
                                "Repair a damaged room by one space on the Repair Track. Discard the matching card."
                                    .to_string(),
                        },
                    ),
                );
                rooms.push(Room::MessHall(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Nanobots, false),
                                (RessourceType::Energy, false),
                                (RessourceType::Data, false),
                            ],
                        },
                        DivertField {
                            RessourceTypes: vec![
                                (RessourceType::Data, false),
                                (RessourceType::Data, false),
                                (RessourceType::Energy, false),
                            ],
                        },
                        Ability {
                            description:
                                "Give, take, or exchange a resource card with another player."
                                    .to_string(),
                        },
                    ),
                );
                rooms.push(Room::Bridge(
                        Version::V1,
                        RepairField {
                            RessourceTypes: vec![
                                (RessourceType::Nanobots, false),
                                (RessourceType::Data, false),
                                (RessourceType::Energy, false),
                            ],
                        },
                        DivertField {   
                            RessourceTypes: vec![
                                (RessourceType::Data, false),
                                (RessourceType::Metal, false),
                                (RessourceType::Energy, false),
                            ],
                        },
                        Ability {
                            description:
                                "Look at the next 3 Damage cards and put them back in any order."
                                    .to_string(),
                        },
                    ),
                );
                rooms.push(Room::EnergyCore(
                        Version::V1,
                        Ability {
                            description:
                                "When all rooms have diverted power, get here and use 1 action to reactivate the Energy Core."
                                    .to_string(),
                        },
                    ),
                );
            }
            Version::V2 => {
                rooms.push(Room::EnergyCore(
                        Version::V1,
                        Ability {
                            description:
                                "When all rooms have diverted power, get here and use 1 action to reactivate the Energy Core."
                                    .to_string(),
                        },
                    ),
                );
            }
        }
    rooms
    }

}

#[derive(Debug)]
pub enum Version {
    V1,
    V2,
}

#[derive(Debug)]
struct RepairField {
    RessourceTypes: Vec<(RessourceType, bool)>, // RessourceType and amount needed to repair
}

#[derive(Debug)]
struct DivertField {
    RessourceTypes: Vec<(RessourceType, bool)>, // RessourceType and amount needed to divert
}

#[derive(Debug)]
struct Ability {
    description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Room {
        pub fn get_room_type(&self) -> String {
            match self {
                Room::CrewQuarters(_, _, _, _) => "Crew Quarters".to_string(),
                Room::EngineRoom(_, _, _, _) => "Engine Room".to_string(),
                Room::Armoury(_, _, _, _) => "Armoury".to_string(),
                Room::MedicalBay(_, _, _, _) => "Medical Bay".to_string(),
                Room::CargoHold(_, _, _, _) => "Cargo Hold".to_string(),
                Room::RepairCenter(_, _, _, _) => "Repair Center".to_string(),
                Room::MessHall(_, _, _, _) => "Mess Hall".to_string(),
                Room::Bridge(_, _, _, _) => "Bridge".to_string(),
                Room::EnergyCore(_, _) => "Energy Core".to_string(),
            }
        }
    }

    #[test]
    fn test_init_rooms() {
        let rooms = Room::init_rooms(Version::V1);
        assert_eq!(rooms.len(), 9, "Expected 9 rooms");

        let room_hash_set: HashSet<String> = rooms
            .iter()
            .map(|room| room.get_room_type())
            .collect();
        assert_eq!(room_hash_set.len(), 9, "Expected 9 unique room types");
        assert!(room_hash_set.contains("Crew Quarters"), "Expected Crew Quarters");
        assert!(room_hash_set.contains("Engine Room"), "Expected Engine Room");
        assert!(room_hash_set.contains("Armoury"), "Expected Armoury");
        assert!(room_hash_set.contains("Medical Bay"), "Expected Medical Bay");
        assert!(room_hash_set.contains("Cargo Hold"), "Expected Cargo Hold");
        assert!(room_hash_set.contains("Repair Center"), "Expected Repair Center");
        assert!(room_hash_set.contains("Mess Hall"), "Expected Mess Hall");
        assert!(room_hash_set.contains("Bridge"), "Expected Bridge");
        assert!(room_hash_set.contains("Energy Core"), "Expected Energy Core");
    }
}