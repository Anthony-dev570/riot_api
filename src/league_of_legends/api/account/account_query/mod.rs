use serde::{Deserialize, Serialize};

pub mod imp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountQuery {
    ByPuuid(String),
    ByRiotId {
        tag_line: String,
        game_name: String
    },
    ByGame {
        game: String,
        puuid: String
    },
    Me {
        authorization: String
    }
}