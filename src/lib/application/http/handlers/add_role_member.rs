use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension};
use serde::Serialize;

use crate::{application::http::auth::UserPayload, domain::member::ports::MemberService};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AddRoleMemberResponseData;

pub async fn add_role_member<M: MemberService>(
    Extension(member_service): Extension<Arc<M>>,
    Extension(_user): Extension<UserPayload>,
    Path(guild_id): Path<String>,
    Path(role_id): Path<String>,
    Path(user_id): Path<String>,
) -> Result<ApiSuccess<AddRoleMemberResponseData>, ApiError> {
    // @TODO Verify if the user has permission to add a role to a member
    member_service
        .add_role(guild_id, user_id, role_id)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::OK, AddRoleMemberResponseData))
}
