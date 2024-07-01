use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::league_of_legends::routing::routing_value::RoutingValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlatformRoutingValue {
    BR1,
    EUN1,
    EUW1,
    JP1,
    KR,
    LA1,
    LA2,
    NA1,
    OC1,
    TR1,
    RU,
    PH2,
    SG2,
    TH2,
    TW2,
    VN2
}

impl Display for PlatformRoutingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl RoutingValue for PlatformRoutingValue {
    fn host(&self) -> String {
        format!("{:?}.api.riotgames.com", self)
    }
}