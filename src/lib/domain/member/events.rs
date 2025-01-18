use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct GuildCreateEvent {
    pub owner_id: String,
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddRoleEvent {
    pub guild_id: String,
    pub member_id: String,
    pub role_id: String,
}
