use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Member {
    pub id: String,
    pub user_id: String,
    pub nick: Option<String>,
    pub roles: Vec<String>,
}

impl Member {
    pub fn new(user_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            nick: None,
            roles: vec![],
        }
    }
}
