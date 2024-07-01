pub mod riot;
pub mod league_of_legends;
pub mod utilities;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::league_of_legends::api::account::account_query::AccountQuery;
    use crate::league_of_legends::api::champion_mastery::champion_mastery_query::ChampionMasteryQuery;
    use crate::league_of_legends::api::clash::clash_query::ClashQuery;
    use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
    use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;
    use crate::riot::Riot;

    #[tokio::test]
    async fn it_works() {
        let lol = Riot::new("xxxxxxxxxxxxxxxxxxxxxxxxxx").league_of_legends(RegionalRoutingValue::Americas, PlatformRoutingValue::NA1);
        let c = lol.account(AccountQuery::ByRiotId {
            tag_line: "oops".to_string(),
            game_name: "Major Motoko".to_string(),
        }).await.unwrap();
        println!("{}", c.puuid());
        let t = lol.champion_mastery(ChampionMasteryQuery::ByPuuidScore(c.puuid().to_string())).await.unwrap();
        println!("{:?}", t);
        //lol.clash(ClashQuery::BySummonerId());
    }
}
