use std::fmt::Display;
use serde::{Deserialize, Serialize};

use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;
use crate::league_of_legends::routing::routing_value::RoutingValue;

pub mod routing_value;
pub mod platform_routing_value;
pub mod regional_routing_value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnyRoutingValue {
    Platform(PlatformRoutingValue),
    Regional(RegionalRoutingValue)
}

impl Display for AnyRoutingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AnyRoutingValue::Platform(p) => p.to_string(),
            AnyRoutingValue::Regional(r) => r.to_string()
        };
        write!(f, "{}", str)
    }
}

impl RoutingValue for AnyRoutingValue {
    fn host(&self) -> String {
        match self {
            AnyRoutingValue::Platform(p) => p.host(),
            AnyRoutingValue::Regional(r) => r.host()
        }
    }
}