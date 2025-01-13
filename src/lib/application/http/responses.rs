use crate::{application::http::handlers::ApiError, domain::member::entities::error::MemberError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiErrorDetail>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorDetail {
    pub message: String,
    pub rule: String,
    pub field: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiResponseError {
    pub code: String,
    pub status: u16,
    pub message: String,
}

impl From<MemberError> for ApiError {
  fn from(value: MemberError) -> Self {
    match value {
      MemberError::NotFound => ApiError::NotFound("Member not found".to_string()),
      MemberError::Unauthorized => ApiError::Unauthorized("Unauthorized".to_string()),
      MemberError::Forbidden => ApiError::Forbidden("Forbidden".to_string()),
      MemberError::CreateError(e) => ApiError::InternalServerError(e),
      MemberError::DeleteError(e) => ApiError::InternalServerError(e),
    }
  }
}