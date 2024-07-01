use crate::league_of_legends::LeagueOfLegends;
use crate::league_of_legends::routing::platform_routing_value::PlatformRoutingValue;
use crate::league_of_legends::routing::regional_routing_value::RegionalRoutingValue;
use crate::riot::Riot;

impl <'a> Riot<'a> {
    pub fn new(key: &'a str) -> Self {
        Self(key)
    }

    pub fn league_of_legends(&self, regional_routing_value: RegionalRoutingValue, platform_routing_value: PlatformRoutingValue) -> LeagueOfLegends<'a> {
        LeagueOfLegends::new(self, regional_routing_value, platform_routing_value)
    }

    pub fn key(&self) -> &'a str {
        self.0
    }
}