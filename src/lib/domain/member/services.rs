use std::sync::Arc;

use crate::application::ports::messaging_ports::{MessagingPort, MessagingTypeImpl};

use super::{
    entities::{error::MemberError, model::Member},
    events::AddRoleEvent,
    ports::{MemberRepository, MemberService},
};

#[derive(Debug, Clone)]
pub struct MemberServiceImpl<M>
where
    M: MemberRepository,
{
    pub member_repository: M,
    pub messaging: Arc<MessagingTypeImpl>,
}

impl<M> MemberServiceImpl<M>
where
    M: MemberRepository,
{
    pub fn new(member_repository: M, messaging: Arc<MessagingTypeImpl>) -> Self {
        Self {
            member_repository,
            messaging,
        }
    }
}

impl<M> MemberService for MemberServiceImpl<M>
where
    M: MemberRepository,
{
    async fn find_by_id(&self, id: &str) -> Result<Member, MemberError> {
        self.member_repository
            .find_by_id(id)
            .await?
            .ok_or(MemberError::NotFound)
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Member, MemberError> {
        self.member_repository
            .find_by_user_id(user_id)
            .await?
            .ok_or(MemberError::NotFound)
    }

    async fn find_by_guild_id(&self, id: &str) -> Result<Vec<Member>, MemberError> {
        self.member_repository.find_by_guild_id(id).await
    }

    async fn create(&self, user_id: String, guild_id: String) -> Result<Member, MemberError> {
        self.member_repository.create(user_id, guild_id).await
    }

    async fn add_role(
        &self,
        guild_id: String,
        user_id: String,
        role_id: String,
    ) -> Result<(), MemberError> {
        let member = self
            .member_repository
            .add_role(guild_id.clone(), user_id, role_id.clone())
            .await?;

        let event = AddRoleEvent {
            guild_id,
            member_id: member.id,
            role_id,
        };

        self.messaging
            .publish_message(String::from(""), event)
            .await
            .map_err(|e| MemberError::AddRoleError(e.to_string()))?;

        Ok(())
    }
}
