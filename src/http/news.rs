use std::sync::Arc;
use anyhow::Context;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use serde_json::json;

use crate::db;
use crate::http::{ApiContext, Result};
use crate::models::{Feed, NewsItem, Source, SourceType};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/source_types", get(get_source_types).post(post_source_types))
        .route("/api/source_types/:id", delete(delete_source_type))
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct CreateSourceTypeRequest {
    name: String,
}

// create an async function to handle post
async fn post_source_types(
    ctx: State<ApiContext>,
    Json(body): Json<CreateSourceTypeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = db::save_source_type(&ctx.db, &body.name).await;

    return match query_result {
        Ok(source_type) => {
            let note_response = json!({"status": "success","data": json!({
                "source_type": source_type
            })});

            Ok((StatusCode::CREATED, Json(note_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": "source_type already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    };
}

async fn delete_source_type(
    ctx: State<ApiContext>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM source_type WHERE id = $1", id)
        .execute(&ctx.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("source_type with id: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

