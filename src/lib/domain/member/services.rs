use super::ports::{MemberRepository, MemberService};

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
    async fn find_by_id(
        &self,
        _id: &str,
    ) -> Result<super::entities::model::Member, super::entities::error::MemberError> {
        unimplemented!()
    }

    async fn find_by_user_id(
        &self,
        _user_id: &str,
    ) -> Result<super::entities::model::Member, super::entities::error::MemberError> {
        unimplemented!()
    }

    async fn find_by_guild_id(
        &self,
        id: &str,
    ) -> Result<Vec<super::entities::model::Member>, super::entities::error::MemberError> {
        self.member_repository.find_by_guild_id(id).await
    }
}
