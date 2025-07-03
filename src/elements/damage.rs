use crate::elements::*;

pub struct Damage {
    room_types: Vec<RoomType>,
}

impl Damage {
    pub fn new(room_types: Vec<RoomType>) -> Self {
        Self { room_types }
    }

    pub fn init() -> (Vec<Self>, Vec<Self>, Vec<Self>) {
        let room_types = vec![
            RoomType::CrewQuarters,
            RoomType::EngineRoom,
            RoomType::Armoury,
            RoomType::MedicalBay,
            RoomType::CargoHold,
            RoomType::RepairCenter,
            RoomType::MessHall,
            RoomType::Bridge,
        ];

        let damage_1 = room_types.iter()
            .map(|r_t| Damage::new(vec![r_t.clone()]))
            .collect::<Vec<Damage>>();

        let damage_2 = vec![
            Damage::new(vec![RoomType::Bridge, RoomType::Armoury]),
            Damage::new(vec![RoomType::EngineRoom, RoomType::RepairCenter]),
            Damage::new(vec![RoomType::Bridge, RoomType::CargoHold]),
            Damage::new(vec![RoomType::RepairCenter, RoomType::CargoHold]),
            Damage::new(vec![RoomType::CrewQuarters, RoomType::MessHall]),
            Damage::new(vec![RoomType::EngineRoom, RoomType::MedicalBay]),
            Damage::new(vec![RoomType::Armoury, RoomType::MessHall]),
            Damage::new(vec![RoomType::MedicalBay, RoomType::CrewQuarters]),
        ];

        let damage_3 = vec![
            Damage::new(vec![RoomType::CrewQuarters, RoomType::Armoury, RoomType::CargoHold]),
            Damage::new(vec![RoomType::MedicalBay, RoomType::CargoHold, RoomType::MessHall]),
            Damage::new(vec![RoomType::EngineRoom, RoomType::Armoury, RoomType::CargoHold]),
            Damage::new(vec![RoomType::Bridge, RoomType::RepairCenter, RoomType::CrewQuarters]),
            Damage::new(vec![RoomType::Bridge, RoomType::EngineRoom, RoomType::CrewQuarters]),
            Damage::new(vec![RoomType::Bridge, RoomType::MedicalBay, RoomType::MessHall]),
            Damage::new(vec![RoomType::MedicalBay, RoomType::RepairCenter, RoomType::Armoury]),
            Damage::new(vec![RoomType::EngineRoom, RoomType::RepairCenter, RoomType::MessHall]),
        ];

        (damage_1, damage_2, damage_3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_initialization() {
        let (damage_1, damage_2, damage_3) = Damage::init();
        
        assert_eq!(damage_1.len(), 8, "Expected 8 single room damages");
        assert_eq!(damage_2.len(), 8, "Expected 8 double room damages");
        assert_eq!(damage_3.len(), 8, "Expected 8 triple room damages");

        
    }
}