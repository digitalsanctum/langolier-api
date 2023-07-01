use crate::config::Config;
use anyhow::Context;
use axum::Router;
use sqlx::PgPool;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::Method;
use reqwest::header::AUTHORIZATION;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod api;
mod error;
mod fetcher;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(config.clone()),
        db,
    };

    let app = api_router(api_context);
    let port = &config.port.or(Some(8080)).unwrap();

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, *port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router(api_context: ApiContext) -> Router {

    // TODO for dev only
    let cors_layer = CorsLayer::new()
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        // .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new()
            .level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new()
            .level(Level::INFO));

    Router::new()
        .merge(fetcher::router())
        .merge(api::router())
        .layer(tracing_layer)
        .layer(cors_layer)
        .with_state(api_context)
}
