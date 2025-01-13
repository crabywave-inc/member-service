use std::sync::Arc;

use axum::http::StatusCode;
use axum::{extract::Path, Extension};
use serde::Serialize;

use crate::{
    application::http::auth::UserPayload,
    domain::member::{entities::model::Member, ports::MemberService},
};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GetMembersGuildResponseData(Vec<Member>);

pub async fn get_members_guild<M: MemberService>(
    Extension(member_service): Extension<Arc<M>>,
    Extension(_user): Extension<UserPayload>,
    Path(guild_id): Path<String>,
) -> Result<ApiSuccess<GetMembersGuildResponseData>, ApiError> {
    // @TODO Verify if the user is a member of the guild
    member_service
        .find_by_guild_id(&guild_id)
        .await
        .map_err(ApiError::from)
        .map(|members| ApiSuccess::new(StatusCode::OK, GetMembersGuildResponseData(members)))
}
