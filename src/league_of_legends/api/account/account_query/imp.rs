use crate::league_of_legends::account::account_query::AccountQuery;
use crate::utilities::to_url::ToUrl;

impl AccountQuery {
    const BASE_URL: &'static str = "https://.api.riotgames.com/riot/account/v1/";
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