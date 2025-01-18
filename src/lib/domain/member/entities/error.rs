use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum MemberError {
    #[error("Member not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Failed to create member: {0}")]
    CreateError(String),
    #[error("Failed to delete member: {0}")]
    DeleteError(String),
    #[error("Failed to add role: {0}")]
    AddRoleError(String),
}
