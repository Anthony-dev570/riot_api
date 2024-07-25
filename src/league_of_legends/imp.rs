use crate::error::Error;
use crate::league_of_legends::api::account::account_dto::AccountDto;
use crate::league_of_legends::api::account::account_query::AccountQuery;
use crate::league_of_legends::api::champion::champion_info::ChampionInfo;
use crate::league_of_legends::api::champion::champion_query::ChampionQuery;
use crate::league_of_legends::api::champion_mastery::champion_mastery_query::ChampionMasteryQuery;
use crate::league_of_legends::api::champion_mastery::champion_mastery_result::ChampionMasteryResult;
use crate::league_of_legends::api::clash::clash_query::ClashQuery;
use crate::league_of_legends::LeagueOfLegends;
use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;
use crate::riot::Riot;

impl<'a> LeagueOfLegends<'a> {
    pub fn new(riot: &Riot<'a>, regional_routing_value: RegionalRoutingValue, platform_routing_value: PlatformRoutingValue) -> Self {
        Self {
            key: riot.key(),
            platform_routing_value,
            regional_routing_value,
        }
    }

    pub async fn account(&self, account_query: AccountQuery) -> Result<AccountDto, Error> {
        account_query.query(self.regional_routing_value, self.key).await
    }

    ///Returns all the data associated with free champion rotation.
    pub async fn champion_rotation(&self) -> Result<ChampionInfo, Error> {
        ChampionQuery {}.query(self.platform_routing_value, self.key).await
    }

    pub async fn champion_mastery(&self, query: ChampionMasteryQuery) -> Result<ChampionMasteryResult, Error> {
        query.query(self.platform_routing_value, self.key).await
    }

    pub async fn clash(&self, query: ClashQuery) -> Result<(), Error> {
        query.query(self.regional_routing_value, self.key).await
    }

    pub fn key(&self) -> &'a str {
        self.key
    }
}