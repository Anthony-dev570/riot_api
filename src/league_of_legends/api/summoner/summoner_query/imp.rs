use crate::league_of_legends::api::summoner::summoner_query::SummonerQuery;
use crate::league_of_legends::api::summoner::summoner_result::SummonerResult;
use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
use crate::utilities::to_url::ToUrl;

///https://developer.riotgames.com/apis#summoner-v4/GET_getByRSOPUUID
impl SummonerQuery {
    const BASE_URL: &'static str = "https://{}.api.riotgames.com/fulfillment/v1/summoners/";

    pub async fn query(&self, routing_value: PlatformRoutingValue, key: &str) -> Result<SummonerResult, crate::error::Error>{
        match self {
            SummonerQuery::ByRSO(_) => {
                let url = format!(
                    "{}?api_key={}", self.to_url().replace("{}", &routing_value.to_string()), key
                );
                println!("{}", url);
            }
            SummonerQuery::EncryptedAccountId(_) => {}
            SummonerQuery::EncryptedPuuid(_) => {}
            SummonerQuery::Me => {}
            SummonerQuery::EncryptedSummonerId(_) => {}
        }

        Ok(SummonerResult::Todo)
    }
}

impl ToUrl for SummonerQuery {
    fn to_url(&self) -> String {
        //ChampionMasteryQuery::ByPuuid(puuid) => format!("{}champion-masteries/by-puuid/{}", Self::BASE_URL, puuid),
        match self {
            SummonerQuery::ByRSO(rso) => format!("{}by-puuid/{}", Self::BASE_URL, rso),
            _ => todo!("Not yet implemented.")
        }
    }
}