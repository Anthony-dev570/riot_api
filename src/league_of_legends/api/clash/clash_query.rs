use serde::{Deserialize, Serialize};
use crate::league_of_legends::api::clash::player_dto::PlayerDto;
use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;
use crate::utilities::to_url::ToUrl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClashQuery {
    BySummonerId(String),
    TeamByTeamId(String),
    Tournaments,
    TournamentByTeamId(String),
    TournamentById(i32),
}

impl ClashQuery {
    const BASE_URL: &'static str = "https://{}.api.riotgames.com/lol/clash/v1/";

    pub async fn query(&self, region: RegionalRoutingValue, key: &str) -> Result<(), crate::error::Error> {
        todo!("Need to implement summoner api before this.");
        let url = format!("{}?api_key={}", self.to_url().replace("{}", &region.to_string()), key);

        match self {
            ClashQuery::BySummonerId(_) => {
                let players: Vec<PlayerDto> = reqwest::get(url).await.map_err(|e| crate::error::Error::Http(e))?.json().await.map_err(|e| crate::error::Error::Http(e))?;
                println!("{:?}", players);
            }
            ClashQuery::TeamByTeamId(_) => {}
            ClashQuery::Tournaments => {}
            ClashQuery::TournamentByTeamId(_) => {}
            ClashQuery::TournamentById(_) => {}
        }

        Ok(())
    }
}

impl ToUrl for ClashQuery {
    fn to_url(&self) -> String {
        match self {
            ClashQuery::BySummonerId(summoner_id) => format!("{}players/by-summoner/{}", Self::BASE_URL, summoner_id),
            ClashQuery::TeamByTeamId(team_id) => format!("{}teams/{}", Self::BASE_URL, team_id),
            ClashQuery::Tournaments => format!("{}tournaments", Self::BASE_URL),
            ClashQuery::TournamentByTeamId(team_id) => format!("{}tournaments/by-team/{}", Self::BASE_URL, team_id),
            ClashQuery::TournamentById(id) => format!("{}tournaments/{}", Self::BASE_URL, id),
        }
    }
}