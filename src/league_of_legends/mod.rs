use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;

pub mod imp;
pub mod api;
pub mod routing;


pub struct LeagueOfLegends<'a> {
    key: &'a str,
    platform_routing_value: PlatformRoutingValue,
    regional_routing_value: RegionalRoutingValue
}