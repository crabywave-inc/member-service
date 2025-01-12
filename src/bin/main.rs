use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use member::{
    application::http::{HttpServer, HttpServerConfig},
    domain::member::services::MemberServiceImpl,
    env::Env,
    infrastructure::member::db::firestore_member_repository::FirestoreMemberRepository,
};
use member::{env::AppEnv, infrastructure::db::firestore::Firestore};

fn init_logger(env: Arc<Env>) {
    match env.env {
        AppEnv::Development => {
            tracing_subscriber::fmt::init();
        }
        AppEnv::Production => {
            tracing_subscriber::fmt()
                .json()
                .with_max_level(tracing::Level::INFO)
                .init();
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let env = Arc::new(Env::parse());

    init_logger(Arc::clone(&env));

    let firestore = Arc::new(Firestore::new(Arc::clone(&env)).await?);

    let member_repository = FirestoreMemberRepository::new(Arc::clone(&firestore));
    let member_service = Arc::new(MemberServiceImpl::new(member_repository));

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config, Arc::clone(&env), Arc::clone(&member_service)).await?;

    http_server.run().await?;

    Ok(())
}
