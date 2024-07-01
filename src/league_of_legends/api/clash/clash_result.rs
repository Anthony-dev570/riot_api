use serde::{Deserialize, Serialize};
use crate::league_of_legends::api::clash::player_dto::PlayerDto;
use crate::league_of_legends::api::clash::team_dto::TeamDto;
use crate::league_of_legends::api::clash::tournament_dto::TournamentDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClashResult {
    Players(Vec<PlayerDto>),
    Team(TeamDto),
    Tournaments(Vec<TournamentDto>),
    Tournament(TournamentDto),
}