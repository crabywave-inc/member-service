use super::{
    entities::{error::MemberError, model::Member},
    ports::{MemberRepository, MemberService},
};

#[derive(Debug, Clone)]
pub struct MemberServiceImpl<M>
where
    M: MemberRepository,
{
    pub member_repository: M,
}

impl<M> MemberServiceImpl<M>
where
    M: MemberRepository,
{
    pub fn new(member_repository: M) -> Self {
        Self { member_repository }
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

    async fn create(&self, user_id: String) -> Result<Member, MemberError> {
        self.member_repository.create(user_id).await
    }
}
