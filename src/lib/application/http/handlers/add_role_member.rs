use axum::{extract::Path, Extension};
use serde::Serialize;

use crate::application::http::auth::UserPayload;

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AddRoleMemberResponseData;

pub async fn add_role_member(
    Extension(_user): Extension<UserPayload>,
    Path(_guild_id): Path<String>,
    Path(_role_id): Path<String>,
    Path(_user_id): Path<String>,
) -> Result<ApiSuccess<AddRoleMemberResponseData>, ApiError> {
    unimplemented!("add_role_member")
}
