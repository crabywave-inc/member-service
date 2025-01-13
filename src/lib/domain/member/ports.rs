use std::future::Future;

use super::entities::{error::MemberError, model::Member};

pub trait MemberService: Clone + Send + Sync + 'static {
    fn find_by_user_id(
        &self,
        user_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn find_by_guild_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Vec<Member>, MemberError>> + Send;
}

pub trait MemberRepository: Clone + Send + Sync + 'static {
    fn find_by_user_id(
        &self,
        user_id: &str,
    ) -> impl Future<Output = Result<Option<Member>, MemberError>> + Send;
    fn find_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Member>, MemberError>> + Send;
    fn find_by_guild_id(
        &self,
        guild_id: &str,
    ) -> impl Future<Output = Result<Vec<Member>, MemberError>> + Send;
}
