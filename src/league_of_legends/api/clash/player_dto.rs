use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDto {
    summoner_id: String,
    team_id: String,
    position: String,
    role: String
}