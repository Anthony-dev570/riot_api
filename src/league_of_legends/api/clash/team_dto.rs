use serde::*;
use crate::league_of_legends::api::clash::player_dto::PlayerDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDto {
    id: String,
    tournament_id: i32,
    name: String,
    icon_id: i32,
    tier: i32,
    captain: String,
    abbreviation: String,
    players: Vec<PlayerDto>
}

impl TeamDto {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn tournament_id(&self) -> i32 {
        self.tournament_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn icon_id(&self) -> i32 {
        self.icon_id
    }
    pub fn tier(&self) -> i32 {
        self.tier
    }
    pub fn captain(&self) -> &str {
        &self.captain
    }
    pub fn abbreviation(&self) -> &str {
        &self.abbreviation
    }
    pub fn players(&self) -> &Vec<PlayerDto> {
        &self.players
    }
}