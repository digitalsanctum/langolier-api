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
use tracing::Subscriber;
use crate::tasks::CompanyPayload;

mod static_support;

mod db;
mod models;
mod http;
mod config;
mod fetcher;
mod extractor;
mod tasks;
mod rating;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::fmt()
        .compact()
        // .json()
        .with_target(false)
        .init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    let db = PgPoolOptions::new()
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;
    tracing::info!("connected to database");

    // using sync NATS client for now
    let nats_client = nats::connect(&config.nats_url)
        .context("could not connect to nats_url")?;
    tracing::info!("connected to nats server");

    tokio::task::spawn({
        let conn = nats_client.clone();
        async move {
            let mut maybe_sub = conn.subscribe("company_created".into());
            match maybe_sub {
                Ok(sub) => {
                    let mut subscriber = sub;
                    tracing::info!("Awaiting messages on company_created");

                    while let Some(message) = subscriber.next() {
                        let maybe_payload = serde_json::from_slice::<CompanyPayload>(&message.data);
                        match maybe_payload {
                            Ok(payload) => {
                                tracing::info!("Received payload {payload:?}");
                                handle_new_company(&payload.company.name).await;
                            },
                            Err(err) => tracing::error!("Error parsing payload: {err}")
                        }
                    }
                }
                Err(err) => tracing::error!("Error subscribing to company_created: {err}")
            }
        }
    });

    http::serve(config, db, nats_client).await?;

    Ok(())
}

async fn handle_new_company(company_name: &String) {
    tracing::info!("Handling new company {company_name}");
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
