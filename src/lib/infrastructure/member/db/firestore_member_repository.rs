use std::sync::Arc;

use crate::{domain::member::ports::MemberRepository, infrastructure::db::firestore::Firestore};

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
    async fn find_by_id(
        &self,
        _id: &str,
    ) -> Result<
        crate::domain::member::entities::model::Member,
        crate::domain::member::entities::error::MemberError,
    > {
        unimplemented!()
    }

    async fn find_by_user_id(
        &self,
        _user_id: &str,
    ) -> Result<
        crate::domain::member::entities::model::Member,
        crate::domain::member::entities::error::MemberError,
    > {
        unimplemented!()
    }
}
