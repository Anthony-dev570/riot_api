use serde::{Deserialize, Serialize};

pub mod imp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDto {
    puuid: String,
    #[serde(rename = "gameName")]
    game_name: Option<String>,
    #[serde(rename = "tagLine")]
    tag_line: Option<String>
}