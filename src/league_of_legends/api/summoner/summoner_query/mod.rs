pub mod imp;

#[derive(Debug, Clone)]
pub enum SummonerQuery {
    ByRSO(String),
    EncryptedAccountId(String),
    EncryptedPuuid(String),
    Me,
    EncryptedSummonerId(String)
}