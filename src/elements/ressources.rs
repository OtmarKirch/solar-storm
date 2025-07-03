#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RessourceType {
    Metal,
    Energy,
    Nanobots,
    Data,
    Universal
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ressource {
    ressource_type: RessourceType,
}

impl Ressource {
    pub fn new(ressource_type: RessourceType) -> Self {
        Self { ressource_type }
    }

    pub fn init(universal_cnt: u32) -> Vec<Self> {
        let mut ressources: Vec<Ressource> = vec![];
        let metal_vec: Vec<Ressource> = vec![Ressource::new(RessourceType::Metal); 15];
        let energy_vec: Vec<Ressource> = vec![Ressource::new(RessourceType::Energy); 15];
        let nanobots_vec: Vec<Ressource> = vec![Ressource::new(RessourceType::Nanobots); 15];
        let data_vec: Vec<Ressource> = vec![Ressource::new(RessourceType::Data); 15];
        let universal_vec: Vec<Ressource> = vec![Ressource::new(RessourceType::Universal); universal_cnt as usize];

        ressources.extend(metal_vec);
        ressources.extend(energy_vec);
        ressources.extend(nanobots_vec);
        ressources.extend(data_vec);
        ressources.extend(universal_vec);
        
        ressources
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ressource_initialization() {
        let ressources = Ressource::init(2);
        assert_eq!(ressources.len(), 62); // 15 Metal + 15 Energy + 15 Nanobots + 15 Data + 2 Universal
        assert_eq!(ressources.iter().filter(|r_t| r_t.ressource_type == RessourceType::Metal).count(), 15);
        assert_eq!(ressources.iter().filter(|r_t| r_t.ressource_type == RessourceType::Energy).count(), 15);
        assert_eq!(ressources.iter().filter(|r_t| r_t.ressource_type == RessourceType::Nanobots).count(), 15);
        assert_eq!(ressources.iter().filter(|r_t| r_t.ressource_type == RessourceType::Data).count(), 15);
        assert_eq!(ressources.iter().filter(|r_t| r_t.ressource_type == RessourceType::Universal).count(), 2);    
    }

}