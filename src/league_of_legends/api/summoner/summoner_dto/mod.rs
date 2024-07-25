pub mod imp;

#[derive(Debug, Clone)]
pub struct SummonerDto {
    account_id: String,
    profile_icon_id: i32,
    revision_date: i64,
    id: String,
    puuid: String,
    summoner_level: u64
}