use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentPhaseDto {
    id: i32,
    registration_time: i64,
    start_time: i64,
    cancelled: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentDto {
    id: i32,
    theme_id: i32,
    name_key: String,
    name_key_secondary: String,
    schedule: Vec<TournamentPhaseDto>
}