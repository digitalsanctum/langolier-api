use anyhow::Context;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use serde_json::json;

use crate::{db};
use crate::db::source_type_by_id;
use crate::http::{ApiContext, Result};
use crate::models::{Company, Feed, NewsItem, Source, SourceType, SourceTypePatch};
use crate::tasks::CompanyPayload;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/source_types", get(list_source_types).post(post_source_types))
        .route("/api/source_types/:id",
               get(get_source_type)
            .delete(delete_source_type)
            .patch(patch_source_type)
        )
        .route("/api/feeds", get(get_feeds))
        .route("/api/sources", get(get_sources))
        .route("/api/news", get(get_news))
        .route("/api/companies", get(get_company).post(post_company))
        .route("/api/companies/:id", delete(delete_company))
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
struct CompanyBody {
    companies: Vec<Company>,
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

async fn get_company(ctx: State<ApiContext>) -> Result<Json<CompanyBody>> {
    let companies = db::companies(&ctx.db).await.context("Failed to get companies").unwrap();
    Ok(Json(CompanyBody {
        companies,
    }))
}

async fn get_sources(ctx: State<ApiContext>) -> Result<Json<SourcesBody>> {
    let sources = db::sources(&ctx.db).await.context("Failed to get sources").unwrap();
    Ok(Json(SourcesBody {
        sources,
    }))
}

async fn list_source_types(ctx: State<ApiContext>) -> Result<Json<SourceTypesBody>> {
    let vec = db::source_types(&ctx.db).await.context("Failed to get news").unwrap();
    Ok(Json(SourceTypesBody {
        types: vec,
    }))
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct CreateSourceTypeRequest {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct CreateCompanyRequest {
    name: String,
}

async fn post_company(
    ctx: State<ApiContext>,
    Json(body): Json<CreateCompanyRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = db::save_company(&ctx.db, &body.name).await;

    return match query_result {
        Ok(company) => {

            let subject = "company_created";
            let payload = CompanyPayload {
                company: company.clone(),
            };
            let payload_bytes = serde_json::to_vec(&json!(payload)).expect("Failed to serialize CompanyPayload");
            ctx.nc.publish(subject.into(), payload_bytes).expect("Failed to publish company_created");

            let company_response = json!({"status": "success","data": json!({
                "company": company
            })});

            Ok((StatusCode::CREATED, Json(company_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": "company already exists",
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

// create an async function to handle post
async fn post_source_types(
    ctx: State<ApiContext>,
    Json(body): Json<CreateSourceTypeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = db::save_source_type(&ctx.db, &body.name).await;

    return match query_result {
        Ok(source_type) => {
            let source_type_response = json!({"status": "success","data": json!({
                "source_type": source_type
            })});

            Ok((StatusCode::CREATED, Json(source_type_response)))
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

async fn get_source_type(ctx: State<ApiContext>,
                         Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = source_type_by_id(&ctx.db, &id).await;

    return match query_result {
        Ok(source_type) => {
            let response = json!(source_type);
            Ok(Json(response))
        }
        Err(_) => {
            let error_response = json!({
                "status": "error",
                "message": format!("source_type with id: {} not found", id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    };
}

async fn patch_source_type(
    ctx: State<ApiContext>,
    Path(id): Path<i32>,
    Json(patch): Json<SourceTypePatch>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = source_type_by_id(&ctx.db, &id).await;

    if query_result.is_err() {
        let error_response = json!({
            "status": "error",
            "message": format!("source_type with id: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let source_type = query_result.unwrap();

    let query_result = db::update_source_type(&ctx.db, &source_type.id, &patch.name).await;

    return match query_result {
        Ok(source_type) => {
            let response = json!({"status": "success","source_type": source_type});
            Ok(Json(response))
        }
        Err(err) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", err)})),
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

async fn delete_company(
    ctx: State<ApiContext>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM company WHERE id = $1", id)
        .execute(&ctx.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("company with id: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

