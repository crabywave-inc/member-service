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

    async fn find_by_user_id_and_guild_id(
        &self,
        user_id: &str,
        guild_id: &str,
    ) -> Result<Member, MemberError> {
        self.firestore
            .db
            .fluent()
            .select()
            .from("members")
            .filter(|q| {
                q.for_all([
                    q.field("user_id").eq(user_id),
                    q.field("guild_id").eq(guild_id),
                ])
            })
            .obj::<Member>()
            .query()
            .await
            .map_err(|_| MemberError::NotFound)?
            .first()
            .cloned()
            .ok_or(MemberError::NotFound)
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

    async fn create(&self, user_id: String, guild_id: String) -> Result<Member, MemberError> {
        let member = Member::new(user_id, guild_id);

        self.firestore
            .db
            .fluent()
            .insert()
            .into("members")
            .document_id(&member.user_id)
            .object(&member)
            .execute::<()>()
            .await
            .map_err(|e| MemberError::CreateError(e.to_string()))?;

        Ok(member)
    }

    async fn add_role(
        &self,
        guild_id: String,
        user_id: String,
        role_id: String,
    ) -> Result<Member, MemberError> {
        let member = self
            .find_by_user_id_and_guild_id(&user_id, &guild_id)
            .await?;

        let mut updated_role_ids = member.role_ids.clone();

        if !updated_role_ids.contains(&role_id.to_string()) {
            updated_role_ids.push(role_id.to_string());
        }

        self.firestore
            .db
            .fluent()
            .update()
            .in_col("members")
            .document_id(&member.id)
            .object(&Member {
                role_ids: updated_role_ids,
                ..member.clone()
            })
            .execute::<()>()
            .await
            .map_err(|e| MemberError::AddRoleError(e.to_string()))?;

        Ok(member)
    }
}
