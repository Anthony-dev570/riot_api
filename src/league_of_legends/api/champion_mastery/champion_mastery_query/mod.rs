use serde::{Deserialize, Serialize};

pub mod imp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChampionMasteryQuery {
    ByPuuid(String),
    ByPuuidChampionId {
        puuid: String,
        champion_id: u16,
    },
    ByPuuidTop(String),
    ByPuuidScore(String)
}