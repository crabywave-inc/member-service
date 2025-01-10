use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use member::{
    application::http::{HttpServer, HttpServerConfig},
    env::Env,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let env = Arc::new(Env::parse());

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await?;

    Ok(())
}
