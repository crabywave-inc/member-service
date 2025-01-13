use std::sync::Arc;

use anyhow::Context;
use auth::AuthenticationLayer;
use axum::routing::{get, put};
use axum::{Extension, Router};
use handlers::add_role_member::add_role_member;
use handlers::get_members_guild::get_members_guild;
use tracing::{info, info_span};

use crate::domain::member::ports::MemberService;
use crate::env::Env;

pub mod auth;
pub mod handlers;
pub mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Clone)]
struct AppState<M>
where
    M: MemberService,
{
    member_service: Arc<M>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: tokio::net::TcpListener,
}

impl HttpServer {
    pub async fn new<M>(
        config: HttpServerConfig,
        env: Arc<Env>,
        member_service: Arc<M>,
    ) -> anyhow::Result<Self>
    where
        M: MemberService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState {
            member_service: Arc::clone(&member_service),
        };

        let auth_layer = AuthenticationLayer::new(env.auth_service_url.clone());

        let router = Router::new()
            .nest("", api_routes())
            .layer(trace_layer)
            .layer(auth_layer)
            .layer(Extension(Arc::clone(&state.member_service)))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!(
            "Server is running on http://{}",
            self.listener.local_addr()?
        );

        axum::serve(self.listener, self.router)
            .await
            .context("received error while running server")?;

        Ok(())
    }
}

fn api_routes<M>() -> axum::Router<AppState<M>>
where
    M: MemberService,
{
    axum::Router::new()
        .route("/guilds/:guild_id/members", get(get_members_guild::<M>))
        .route(
            "/guilds/:guild_id/members/:user_id/roles/:role_id",
            put(add_role_member),
        )
}
