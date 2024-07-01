use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::league_of_legends::routing::routing_value::RoutingValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RegionalRoutingValue {
    Americas,
    Asia,
    Europe,
    Sea
}

impl Display for RegionalRoutingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl RoutingValue for RegionalRoutingValue {
    fn host(&self) -> String {
        format!("{:?}.api.riotgames.com", self)
    }
}