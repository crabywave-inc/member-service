use std::sync::Arc;

use crate::{
    domain::member::{
        entities::{error::MemberError, model::Member},
        ports::MemberRepository,
    },
    infrastructure::db::firestore::Firestore,
};

#[derive(Debug, Clone)]
pub struct FirestoreMemberRepository {
    pub firestore: Arc<Firestore>,
}

impl FirestoreMemberRepository {
    pub fn new(firestore: Arc<Firestore>) -> Self {
        Self { firestore }
    }
}

impl MemberRepository for FirestoreMemberRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Member>, MemberError> {
        let member: Option<Member> = self
            .firestore
            .db
            .fluent()
            .select()
            .by_id_in("members")
            .obj()
            .one(id)
            .await
            .map_err(|_| MemberError::NotFound)?;

        Ok(member)
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Member>, MemberError> {
        let members = self
            .firestore
            .db
            .fluent()
            .select()
            .from("members")
            .filter(|q| q.for_all([q.field("user_id").eq(user_id)]))
            .obj::<Member>()
            .query()
            .await
            .map_err(|_| MemberError::NotFound)?;

        let member = members.first().cloned();

        Ok(member)
    }

    async fn find_by_guild_id(&self, guild_id: &str) -> Result<Vec<Member>, MemberError> {
        let members = self
            .firestore
            .db
            .fluent()
            .select()
            .from("members")
            .filter(|q| q.for_all([q.field("guild_id").eq(guild_id)]))
            .obj::<Member>()
            .query()
            .await
            .map_err(|_| MemberError::NotFound)?;

        Ok(members)
    }
}
