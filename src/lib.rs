pub mod riot;
pub mod league_of_legends;
pub mod utilities;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::league_of_legends::api::account::account_query::AccountQuery;
    use crate::league_of_legends::api::summoner::summoner_query::SummonerQuery;
    use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
    use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;
    use crate::riot::Riot;

    #[tokio::test]
    async fn it_works() {
        let lol = Riot::new("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").league_of_legends(RegionalRoutingValue::Americas, PlatformRoutingValue::NA1);

        SummonerQuery::ByRSO("RSO".to_string()).query(PlatformRoutingValue::NA1, lol.key()).await.unwrap();
        let c = lol.account(AccountQuery::ByRiotId {
            tag_line: "oops".to_string(),
            game_name: "Major Motoko".to_string(),
        }).await.unwrap();



        println!("{:?}", c);
    }
}