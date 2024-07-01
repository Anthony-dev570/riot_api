use crate::league_of_legends::api::account::account_dto::AccountDto;
use crate::league_of_legends::api::account::account_query::AccountQuery;
use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;
use crate::utilities::to_url::ToUrl;

impl AccountQuery {
    const BASE_URL: &'static str = "https://{}.api.riotgames.com/riot/account/v1/";

    pub async fn query(&self, routing_value: RegionalRoutingValue, key: &str) -> Result<AccountDto, crate::error::Error> {
        let url = format!("{}?api_key={}", self.to_url().replace("{}", &routing_value.to_string()).replace(" ", "%20"), key);

        let me: AccountDto = reqwest::get(url).await.map_err(|e| crate::error::Error::Http(e))?.json().await.map_err(|e| crate::error::Error::Http(e))?;
        Ok(me)
    }
}

impl ToUrl for AccountQuery {
    fn to_url(&self) -> String {
        format!("{}{}", Self::BASE_URL, match self {
            AccountQuery::ByPuuid(puuid) => format!("accounts/by-puuid/{puuid}"),
            AccountQuery::ByRiotId { tag_line, game_name } => format!("accounts/by-riot-id/{game_name}/{tag_line}"),
            AccountQuery::ByGame { game, puuid } => {
                format!("active-shards/by-game/{game}/by-puuid/{puuid}")
            }
            AccountQuery::Me { .. } => {
                todo!("Me is currently not available.")
            }
        })
    }
}