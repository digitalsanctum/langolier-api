use anyhow::Context;
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;

use crate::db;
use crate::http::{ApiContext, Result};
use crate::models::{Feed, NewsItem, Source, SourceType};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/source_types", get(get_source_types))
        .route("/api/feeds", get(get_feeds))
        .route("/api/sources", get(get_sources))
        .route("/api/news", get(get_news))
}

#[derive(serde::Serialize)]
struct SourceTypesBody {
    types: Vec<SourceType>,
}

#[derive(serde::Serialize)]
struct FeedsBody {
    feeds: Vec<Feed>,
}

#[derive(serde::Serialize)]
struct NewsBody {
    news: Vec<NewsItem>,
}

#[derive(serde::Serialize)]
struct SourcesBody {
    sources: Vec<Source>,
}

async fn get_feeds(ctx: State<ApiContext>) -> Result<Json<FeedsBody>> {
    let feeds = db::feeds(&ctx.db).await.context("Failed to get feeds").unwrap();
    Ok(Json(FeedsBody {
        feeds,
    }))
}
async fn get_news(ctx: State<ApiContext>) -> Result<Json<NewsBody>> {
    let news = db::news(&ctx.db).await.context("Failed to get news").unwrap();
    Ok(Json(NewsBody {
        news,
    }))
}

async fn get_sources(ctx: State<ApiContext>) -> Result<Json<SourcesBody>> {
    let sources = db::sources(&ctx.db).await.context("Failed to get sources").unwrap();
    Ok(Json(SourcesBody {
        sources,
    }))
}

async fn get_source_types(ctx: State<ApiContext>) -> Result<Json<SourceTypesBody>> {
    let vec = db::source_types(&ctx.db).await.context("Failed to get news").unwrap();
    Ok(Json(SourceTypesBody {
        types: vec,
    }))
}
