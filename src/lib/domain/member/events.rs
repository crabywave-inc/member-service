use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GuildCreateEvent {
    pub owner_id: String,
    pub name: String,
    pub id: String,
}
