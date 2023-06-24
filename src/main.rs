use std::net::SocketAddr;
use anyhow::Context;

use axum::{
    http::StatusCode,
    Router,
    routing::get,
};
use axum::extract::State;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use static_support::using_serve_dir;
use crate::config::Config;
use crate::models::{Source, SourceType};

use clap::Parser;
mod static_support;

mod db;
mod models;
mod http;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    http::serve(config, db).await?;

    Ok(())

    /*let app = Router::new()
        .route("/", get(home_handler))
        .with_state(pool)
        ;


    tokio::join!(
        
        serve(using_serve_dir(), 3000),
        
        serve(app, 8080)
    );*/
}

async fn home_handler(State(pool): State<PgPool>) -> Result<Vec<SourceType>, (StatusCode, String)> {
    db::source_types(&pool)
        .await
        .map_err(internal_error)
}

async fn serve(app: Router, port: u16) {
    let addr_str = format!("[::]:{}", port);
    tracing::info!("listening on {}", addr_str);
    let addr = addr_str.parse::<SocketAddr>().expect("invalid address");
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}


/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String) where E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
