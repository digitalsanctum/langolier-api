use std::io::Error;

use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::json;
use webpage::Webpage;

use crate::fetcher::fetch_url;
use crate::http::{ApiContext, Result};
use crate::models::{WebpageRequest, WebpageResponse};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/webpages", get(get_webpages).post(post_webpage))
}

#[derive(serde::Serialize)]
struct FetchBody {
    fetches: Vec<WebpageResponse>,
}

async fn get_webpages(ctx: State<ApiContext>) -> Result<Json<FetchBody>> {

    // TODO: get fetches from db

    let fetches = vec![];
    Ok(Json(FetchBody {
        fetches,
    }))
}

async fn post_webpage(ctx: State<ApiContext>,
                      Json(body): Json<WebpageRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let fetch_result: Result<Webpage, Error> = fetch_url(&body).await;
    return match fetch_result {
        Ok(webpage) => {
            let response = WebpageResponse::new(body, webpage);
            let fetch_response = json!({"status": "success","response": response});
            Ok((StatusCode::OK, Json(fetch_response)))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    };
}