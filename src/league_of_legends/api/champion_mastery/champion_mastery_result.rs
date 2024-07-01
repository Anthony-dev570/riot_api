use serde::{Deserialize, Serialize};
use crate::league_of_legends::api::champion_mastery::champion_mastery_dto::ChampionMasteryDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChampionMasteryResult {
    ChampionMastery(Vec<ChampionMasteryDto>),
    Score(i32)
}