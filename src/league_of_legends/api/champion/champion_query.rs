use crate::league_of_legends::api::champion::champion_info::ChampionInfo;
use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
use crate::utilities::to_url::ToUrl;

pub struct ChampionQuery;

impl ChampionQuery {
    const BASE_URL: &'static str = "https://{}.api.riotgames.com/lol/platform/v3/champion-rotations";

    pub async fn query(&self, routing_value: PlatformRoutingValue, key: &str) -> Result<ChampionInfo, crate::error::Error> {
        reqwest::get(format!("{}?api_key={}", self.to_url().replace("{}", &routing_value.to_string()), key)).await.map_err(|e| crate::error::Error::Http(e))?.json().await.map_err(|e| crate::error::Error::Http(e))
    }
}


impl ToUrl for ChampionQuery {
    fn to_url(&self) -> String {
        Self::BASE_URL.to_string()
    }
}