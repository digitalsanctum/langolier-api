use std::env;
use sqlx::{Error, Pool, Postgres, query_as};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use crate::models::{Company, Feed, Garden, NewsItem, Page, Source, SourceType, Tool};

#[allow(dead_code)]
pub(crate) async fn companies(pool: &Pool<Postgres>) -> Result<Vec<Company>, Error> {
    query_as!(Company, "select * from company order by name")
        .fetch_all(&*pool)
        .await
}

#[allow(dead_code)]
pub(crate) async fn source_types(pool: &Pool<Postgres>) -> Result<Vec<SourceType>, Error> {
    query_as!(SourceType, "select * from source_type")
        .fetch_all(&*pool)
        .await
}

#[allow(dead_code)]
pub(crate) async fn tools(pool: &Pool<Postgres>) -> Result<Vec<Tool>, Error> {
    query_as!(Tool, r#"SELECT * FROM tool"#)
        .fetch_all(&*pool)
        .await
}

#[allow(dead_code)]
pub(crate) async fn sources(pool: &Pool<Postgres>) -> Result<Vec<Source>, Error> {
    query_as!(Source, r#"SELECT * FROM source"#)
        .fetch_all(&*pool)
        .await
}

pub(crate) async fn garden_by_slug(pool: &Pool<Postgres>, slug: &String) -> Result<Garden, Error> {
    query_as!(Garden, r#"SELECT * FROM garden WHERE slug = $1"#, slug)
        .fetch_one(&*pool)
        .await
}

pub(crate) async fn garden_by_id(pool: &Pool<Postgres>, id: &uuid::Uuid) -> Result<Garden, Error> {
    query_as!(Garden, r#"SELECT * FROM garden WHERE id = $1"#, id)
        .fetch_one(&*pool)
        .await
}

pub(crate) async fn pages_by_garden_slug(pool: &Pool<Postgres>, slug: &String) -> Result<Vec<Page>, Error> {
    query_as!(Page, r#"SELECT page.*
FROM garden
inner join page on page.garden_id = garden.id
WHERE garden.slug = $1"#, slug)
        .fetch_all(&*pool)
        .await
}

pub(crate) async fn pages_by_garden_slug_and_type(pool: &Pool<Postgres>, slug: &String, page_type: &String) -> Result<Vec<Page>, Error> {
    query_as!(Page, r#"SELECT page.*
FROM garden
inner join page on page.garden_id = garden.id
WHERE garden.slug = $1 and page.page_type = $2"#, slug, page_type)
        .fetch_all(&*pool)
        .await
}

pub(crate) async fn page_by_id(pool: &Pool<Postgres>, id: &uuid::Uuid) -> Result<Page, Error> {
    query_as!(Page, r#"SELECT * FROM page WHERE id = $1"#, id)
        .fetch_one(&*pool)
        .await
}

pub(crate) async fn source_type_by_id(pool: &Pool<Postgres>, id: &i32) -> Result<SourceType, Error> {
    query_as!(SourceType, r#"SELECT * FROM source_type WHERE id = $1"#, id)
        .fetch_one(&*pool)
        .await
}

pub(crate) async fn update_source_type(pool: &Pool<Postgres>, id: &i32, name: &String) -> Result<SourceType, Error> {
    query_as!(SourceType, r#"UPDATE source_type SET name = $1 WHERE id = $2 RETURNING *"#, name, id)
        .fetch_one(&*pool)
        .await
}

pub(crate) async fn delete_source_type(pool: &Pool<Postgres>, id: &i32) -> Result<PgQueryResult, Error> {
    sqlx::query!("DELETE FROM source_type WHERE id = $1", id)
        .execute(&*pool)
        .await
}

pub(crate) async fn gardens(pool: &Pool<Postgres>) -> Result<Vec<Garden>, Error> {
    query_as!(Garden, r#"SELECT * FROM garden"#)
        .fetch_all(&*pool)
        .await
}

pub(crate) async fn pages(pool: &Pool<Postgres>) -> Result<Vec<Page>, Error> {
    query_as!(Page, r#"SELECT * FROM page"#)
        .fetch_all(&*pool)
        .await
}

#[allow(dead_code)]
pub(crate) async fn feeds(pool: &Pool<Postgres>) -> Result<Vec<Feed>, Error> {
    query_as!(Feed, r#"SELECT * FROM feed"#)
        .fetch_all(&*pool)
        .await
}

#[allow(dead_code)]
pub(crate) async fn news(pool: &Pool<Postgres>) -> Result<Vec<NewsItem>, Error> {
    query_as!(NewsItem, r#"SELECT * FROM news"#)
        .fetch_all(&*pool)
        .await
}

pub(crate) async fn save_company(pool: &Pool<Postgres>, name: &String, url: Option<String>) -> Result<Company, Error> {
    return sqlx::query_as!(Company, "INSERT INTO company (name, url) VALUES ($1, $2) RETURNING *", name, url.unwrap_or("".to_string()))
        .fetch_one(pool)
        .await
}

pub(crate) async fn save_source_type(pool: &Pool<Postgres>, name: &String) -> Result<SourceType, Error> {
    return sqlx::query_as!(SourceType, "INSERT INTO source_type (name) VALUES ($1) RETURNING *", name)
        .fetch_one(pool)
        .await
}

pub(crate) async fn get_pool() -> Pool<Postgres> {
    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(200)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .idle_timeout(std::time::Duration::from_secs(3))
        .connect(db_url.as_str())
        .await
        .expect("Failed to connect to Postgres")
}

pub(crate) async fn save_source(source: &Source, pool: &Pool<Postgres>) -> anyhow::Result<uuid::Uuid> {
    let rec = sqlx::query!(r#"
WITH e AS(
INSERT INTO source (id, name, url, type_id, paywall, feed_available, description, short_name, state, city, create_timestamp)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (url) DO NOTHING
RETURNING id
)
SELECT * FROM e UNION SELECT id FROM source WHERE url = $3
"#,
        source.id, source.name, source.url, source.type_id, source.paywall, source.feed_available, source.description, source.short_name, source.state, source.city, source.create_timestamp)
        .fetch_one(pool)
        .await?;
    Ok(rec.id.unwrap())
}

pub(crate) async fn save_feed(feed: &Feed, pool: &Pool<Postgres>) -> anyhow::Result<uuid::Uuid> {
    let rec = sqlx::query!(r#"
WITH e AS(
INSERT INTO feed (id, url, title, source_id, feed_type)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (url) DO NOTHING
RETURNING id
)
SELECT * FROM e UNION SELECT id FROM feed WHERE url = $2
    "#,
        feed.id, feed.url, feed.title, feed.source_id, feed.feed_type)
        .fetch_one(pool)
        .await?;
    Ok(rec.id.unwrap())
}

pub(crate) async fn save_news_item(ni: &NewsItem, pool: &Pool<Postgres>) -> anyhow::Result<uuid::Uuid> {
    let rec = sqlx::query!(r#"
WITH e AS(
INSERT INTO news (id, title, url, published_timestamp, guid, feed_id)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (guid) DO NOTHING
RETURNING id
)
SELECT * FROM e UNION SELECT id FROM news WHERE guid = $5
        "#,
        ni.id, ni.title, ni.url, ni.published_timestamp, ni.guid, ni.feed_id)
        .fetch_one(pool)
        .await?;
    Ok(rec.id.unwrap())
}