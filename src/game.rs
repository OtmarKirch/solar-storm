use crate::elements::rooms::*;

pub struct GameMeta {
    difficulty: Difficulty,
    version: Version,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Veteran,
    Realist,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Version {
    V1,
    V2,
}