use chrono::Utc;
use sqlx::{Error, Pool, Postgres};
use webpage::Webpage;
use crate::db;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Page {
    pub id : uuid::Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub garden_id: uuid::Uuid,
    pub published: bool,
    pub create_timestamp: chrono::DateTime<Utc>,
    pub update_timestamp: chrono::DateTime<Utc>,
    pub page_type: String,
}

impl Page {
    pub fn new(title: String,
               slug: String,
               content: String,
               garden_id: uuid::Uuid,
               published: bool,
               create_timestamp: chrono::DateTime<Utc>,
               update_timestamp: chrono::DateTime<Utc>,
               page_type: String
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            slug,
            content,
            garden_id,
            published,
            create_timestamp,
            update_timestamp,
            page_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Garden {
    pub id : uuid::Uuid,
    pub title: String,
    pub slug: String,
    pub create_timestamp: chrono::DateTime<Utc>,
    pub update_timestamp: chrono::DateTime<Utc>,
}

impl Garden {
    pub fn new(title: String, slug: String, create_timestamp: chrono::DateTime<Utc>, update_timestamp: chrono::DateTime<Utc>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            slug,
            create_timestamp,
            update_timestamp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct Company {
    pub id: uuid::Uuid,
    pub name: String,
    pub url: Option<String>,
    pub ticker: Option<String>,
    pub indeed_rating: Option<String>,
    pub glassdoor_rating: Option<String>,
    pub create_timestamp: chrono::DateTime<Utc>,
    pub update_timestamp: chrono::DateTime<Utc>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub address: Option<String>,
    pub exchange: Option<String>,
    pub num_employees_min: Option<i32>,
    pub num_employees_max: Option<i32>
}

impl Company {
    pub fn new(name: String,
               url: Option<String>,
               ticker: Option<String>,
               indeed_rating: Option<String>,
               glassdoor_rating: Option<String>,
               create_timestamp: chrono::DateTime<Utc>,
               update_timestamp: chrono::DateTime<Utc>
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            url,
            ticker,
            indeed_rating,
            glassdoor_rating,
            create_timestamp,
            update_timestamp,
            sector: None,
            industry: None,
            address: None,
            exchange: None,
            num_employees_min: None,
            num_employees_max: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub(crate) struct WebpageRequest {
    pub url: String,
    pub persist: bool,
}

impl WebpageRequest {
    pub fn new(url: String, persist: bool) -> Self {
        Self {
            url,
            persist,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct WebpageResponse {
    pub request: WebpageRequest,
    pub webpage: Webpage,
    pub create_timestamp: chrono::DateTime<Utc>,
}

impl WebpageResponse {
    pub fn new(request: WebpageRequest, webpage: Webpage) -> Self {
        Self {
            request,
            webpage,
            create_timestamp: Utc::now(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, serde::Serialize)]
pub(crate) struct SourceType {
    pub id: i32,
    pub name: String,
}

#[allow(dead_code)]
impl SourceType {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    pub async fn save(&self, pool: &Pool<Postgres>) -> Result<SourceType, Error> {
        db::save_source_type(pool, &self.name).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub(crate) struct SourceTypePatch {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize)]
pub(crate) struct Tool {
    pub id: uuid::Uuid,
    pub name: String,
    pub create_timestamp: chrono::DateTime<Utc>,
    pub update_timestamp: chrono::DateTime<Utc>,
}

impl Tool {
    pub fn new(name: String, create_timestamp: chrono::DateTime<Utc>, update_timestamp: chrono::DateTime<Utc>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            create_timestamp,
            update_timestamp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize)]
pub(crate) struct Source {
    pub id: uuid::Uuid,
    pub name: String,
    pub url: String,
    pub type_id: i32,
    pub paywall: Option<bool>,
    pub feed_available: Option<bool>,
    pub description: Option<String>,
    pub short_name: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub create_timestamp: chrono::DateTime<Utc>,
}

impl Source {
    pub fn new(name: String, url: String, type_id: i32) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            url,
            type_id,
            paywall: None,
            feed_available: None,
            description: None,
            short_name: None,
            state: None,
            city: None,
            create_timestamp: Utc::now().into(),
        }
    }

    pub async fn save(&self, pool: &Pool<Postgres>) -> anyhow::Result<uuid::Uuid> {
        db::save_source(self, pool).await
    }
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize)]
pub(crate) struct Feed {
    pub id: uuid::Uuid,
    pub source_id: uuid::Uuid,
    pub url: String,
    pub title: Option<String>,
    pub create_timestamp: chrono::DateTime<Utc>,
    pub feed_type: Option<String>,
    pub ttl: Option<i32>,
}

impl Feed {
    pub fn new(source_id: uuid::Uuid, url: String, title: Option<String>, feed_type: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            source_id,
            url,
            title,
            create_timestamp: Utc::now().into(),
            feed_type,
            ttl: None,
        }
    }

    pub async fn save(&self, pool: &Pool<Postgres>) -> anyhow::Result<uuid::Uuid> {
        db::save_feed(self, pool).await
    }
}


#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize)]
pub(crate) struct NewsItem {
    pub id: uuid::Uuid,
    pub feed_id: uuid::Uuid,
    pub guid: String,
    pub title: String,
    pub published_timestamp: chrono::DateTime<Utc>,
    pub url: String,
    pub create_timestamp: chrono::DateTime<Utc>,
    pub raw_content_path: Option<String>,
    pub text_content_path: Option<String>,
}

impl NewsItem {
    pub fn new(
        feed_id: uuid::Uuid,
        guid: String,
        title: String,
        published_timestamp: chrono::DateTime<Utc>,
        url: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            feed_id,
            guid,
            title,
            published_timestamp,
            url,
            create_timestamp: Utc::now().into(),
            raw_content_path: None,
            text_content_path: None,
        }
    }

    pub async fn save(&self, pool: &Pool<Postgres>) -> anyhow::Result<uuid::Uuid> {
        db::save_news_item(self, pool).await
    }
}