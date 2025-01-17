use std::sync::Arc;

use anyhow::Result;
use firestore::{FirestoreDb, FirestoreDbOptions};

use crate::env::Env;

#[derive(Debug, Clone)]
pub struct Firestore {
    pub db: FirestoreDb,
}

impl Firestore {
    pub async fn new(env: Arc<Env>) -> Result<Self> {
        let db = FirestoreDb::with_options(
            FirestoreDbOptions::new(env.firebase_project_id.clone())
                .with_database_id(env.firebase_database.clone()),
        )
        .await?;

        Ok(Self { db })
    }
}
