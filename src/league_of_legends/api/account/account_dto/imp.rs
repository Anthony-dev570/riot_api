use crate::league_of_legends::api::account::account_dto::AccountDto;

impl AccountDto {
    pub fn puuid(&self) -> &str {
        &self.puuid
    }
    pub fn game_name(&self) -> &Option<String> {
        &self.game_name
    }
    pub fn tag_line(&self) -> &Option<String> {
        &self.tag_line
    }
}