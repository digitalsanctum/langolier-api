use axum::extract::State;
use axum::{Json, Router};
use axum::routing::get;
use crate::http::{ApiContext, Result};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/fetch", get(get_fetches))
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize)]
struct Fetch {
    id: i32,
    url: String
}

#[derive(serde::Serialize)]
struct FetchBody {
    fetches: Vec<Fetch>,
}

async fn get_fetches(ctx: State<ApiContext>) -> Result<Json<FetchBody>> {
    let fetch = Fetch {
        id: 1,
        url: "https://www.google.com".to_string()
    };
    let fetches = vec![fetch];
    Ok(Json(FetchBody {
        fetches,
    }))
}