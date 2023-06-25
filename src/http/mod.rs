use crate::config::Config;
use anyhow::Context;
use axum::Router;
use sqlx::PgPool;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

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
        .allow_headers([CONTENT_TYPE])
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap());
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .merge(fetcher::router())
        .merge(api::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .with_state(api_context)
}
