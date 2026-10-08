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

pub async fn film_exists_for_chat(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    chat_id: i64,
) -> Result<bool> {
    let exists = sqlx::query_scalar(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM films WHERE kinopoisk_id = ? AND telegram_id IN (
                SELECT telegram_id FROM chat_members WHERE chat_id = ?
            )
        )
        "#,
    )
    .bind(kinopoisk_id)
    .bind(chat_id)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn film_exists_for_user(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    telegram_id: i64,
) -> Result<bool> {
    let exists = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM films WHERE kinopoisk_id = ? AND telegram_id = ?)",
    )
    .bind(kinopoisk_id)
    .bind(telegram_id)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn count_user_films(pool: &SqlitePool, telegram_id: i64) -> Result<i64> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM films WHERE telegram_id = ?")
        .bind(telegram_id)
        .fetch_one(pool)
        .await?;

    Ok(count.0)
}

pub async fn count_chat_films(pool: &SqlitePool, chat_id: i64) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM FILMS f
        INNER JOIN chat_members cm ON f.telegram_id = cm.telegram_id
        WHERE cm.chat_id = ?
        "#,
    )
    .bind(chat_id)
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

pub async fn delete_film_for_chat(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    chat_id: i64,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        DELETE FROM films
        WHERE kinopoisk_id = ? 
          AND telegram_id IN (
              SELECT telegram_id 
              FROM chat_members 
              WHERE chat_id = ?
          )
        "#,
    )
    .bind(kinopoisk_id)
    .bind(chat_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_film_for_user(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    telegram_id: i64,
) -> Result<bool> {
    let result = sqlx::query("DELETE FROM films WHERE kinopoisk_id = ? AND telegram_id = ?")
        .bind(kinopoisk_id)
        .bind(telegram_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
