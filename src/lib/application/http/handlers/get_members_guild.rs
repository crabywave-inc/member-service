use std::sync::Arc;

use axum::{extract::Path, Extension};

use crate::domain::member::ports::MemberService;

use super::{ApiError, ApiSuccess};

pub async fn get_members_guild<M: MemberService>(
    Extension(member_service): Extension<Arc<M>>,
    Path(guild_id): Path<String>,
) -> Result<ApiSuccess<String>, ApiError> {
    todo!()
}
