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
    fn create(
        &self,
        user_id: String,
        guild_id: String,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn add_role(
        &self,
        guild_id: String,
        user_id: String,
        role_id: String,
    ) -> impl Future<Output = Result<(), MemberError>> + Send;
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
    fn find_by_user_id_and_guild_id(
        &self,
        user_id: &str,
        guild_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn find_by_guild_id(
        &self,
        guild_id: &str,
    ) -> impl Future<Output = Result<Vec<Member>, MemberError>> + Send;
    fn create(
        &self,
        user_id: String,
        guild_id: String,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn add_role(
        &self,
        guild_id: String,
        user_id: String,
        role_id: String,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
}
