use crate::league_of_legends::api::champion_mastery::champion_mastery_query::ChampionMasteryQuery;
use crate::league_of_legends::api::champion_mastery::champion_mastery_result::ChampionMasteryResult;
use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
use crate::utilities::to_url::ToUrl;

impl ChampionMasteryQuery {
    const BASE_URL: &'static str = "https://{}.api.riotgames.com/lol/champion-mastery/v4/";

    pub async fn query(&self, routing_value: PlatformRoutingValue, key: &str) -> Result<ChampionMasteryResult, crate::error::Error>{
        let url = format!("{}?api_key={}", self.to_url().replace("{}", &routing_value.to_string()), key);

        return match self {
            ChampionMasteryQuery::ByPuuidScore(_) => {
                Ok(ChampionMasteryResult::Score(reqwest::get(url).await.map_err(|e| crate::error::Error::Http(e))?.json().await.map_err(|e|crate::error::Error::Http(e))?))
            }
            _ => {
                Ok(ChampionMasteryResult::ChampionMastery(reqwest::get(url).await.map_err(|e| crate::error::Error::Http(e))?.json().await.map_err(|e|crate::error::Error::Http(e))?))
            }
        }
    }
}

impl ToUrl for ChampionMasteryQuery {
    fn to_url(&self) -> String {
        match self {
            ChampionMasteryQuery::ByPuuid(puuid) => format!("{}champion-masteries/by-puuid/{}", Self::BASE_URL, puuid),
            ChampionMasteryQuery::ByPuuidChampionId { puuid, champion_id } => format!("{}champion-masteries/by-puuid/{}by-champion/{}", Self::BASE_URL, puuid, champion_id),
            ChampionMasteryQuery::ByPuuidTop(puuid) => format!("{}champion-masteries/by-puuid/{}/top", Self::BASE_URL, puuid),
            ChampionMasteryQuery::ByPuuidScore(puuid) => format!("{}scores/by-puuid/{}", Self::BASE_URL, puuid),
        }
    }
}