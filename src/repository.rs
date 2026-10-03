use anyhow::Result;
use sqlx::SqlitePool;

use crate::integrations::movie_api::MovieMetadata;

pub async fn add_film(
    pool: &SqlitePool,
    telegram_id: i64,
    username: Option<String>,
    metadata: &MovieMetadata,
) -> Result<i64> {
    let result = sqlx::query(
        r#"
        INSERT INTO films (telegram_id, username, title, original_title, year, description, poster_url, kinopoisk_id, kinopoisk_url, is_seen, metadata_json)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(telegram_id)
    .bind(username)
    .bind(metadata.title.clone())
    .bind(metadata.original_title.clone())
    .bind(metadata.year)
    .bind(metadata.description.clone())
    .bind(metadata.poster_url.clone())
    .bind(metadata.kinopoisk_id)
    .bind(metadata.kinopoisk_url.clone())
    .bind(false)
    .bind(metadata.raw_data.clone())
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn film_exists(pool: &SqlitePool, kinopoisk_id: i64) -> Result<bool> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM films WHERE kinopoisk_id = ?",
    )
    .bind(kinopoisk_id)
    .fetch_one(pool)
    .await?;
    Ok(count.0 > 0)
}
