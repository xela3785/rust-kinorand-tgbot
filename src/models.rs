use sqlx::FromRow;
use serde_json::Value;

#[derive(Debug, Clone, FromRow)]
pub struct Film {
    pub id: i64,
    pub telegram_id: i64,
    pub username: Option<String>,
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub year: Option<i32>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub kinopoisk_id: i64,
    pub is_seen: bool,
    pub kinopoisk_url: String,
    pub metadata_json: Value,
}