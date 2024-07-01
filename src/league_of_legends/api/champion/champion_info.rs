use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    max_new_player_level: i32,
    free_champion_ids_for_new_players: Vec<i32>,
    free_champion_ids: Vec<i32>
}

impl ChampionInfo {
    pub fn max_new_player_level(&self) -> i32 {
        self.max_new_player_level
    }
    pub fn free_champion_ids_for_new_players(&self) -> &Vec<i32> {
        &self.free_champion_ids_for_new_players
    }
    pub fn free_champion_ids(&self) -> &Vec<i32> {
        &self.free_champion_ids
    }
}