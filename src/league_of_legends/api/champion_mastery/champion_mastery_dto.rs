use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardConfigDto {
    reward_value: String,
    reward_type: String,
    maximum_reward: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextSeasonMilestonesDto {
    require_grade_counts: HashMap<String, u32>,
    reward_marks: u32,
    bonus: bool,
    reward_config: Option<RewardConfigDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryDto {
    puuid: String,
    champion_points_until_next_level: i64,
    #[serde(default)]
    chest_granted: bool,
    champion_id: u64,
    last_play_time: u64,
    champion_level: u32,
    champion_points: u32,
    champion_points_since_last_level: u64,
    mark_required_for_next_level: u32,
    champion_season_milestone: u32,
    next_season_milestone: Option<NextSeasonMilestonesDto>,
    tokens_earned: u32,
    #[serde(default = "Vec::new")]
    milestone_grades: Vec<String>,
}