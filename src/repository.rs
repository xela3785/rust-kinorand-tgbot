use anyhow::Result;
use sqlx::SqlitePool;

use crate::integrations::movie_api::MovieMetadata;
use crate::models::Film;

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
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM films WHERE kinopoisk_id = ?")
        .bind(kinopoisk_id)
        .fetch_one(pool)
        .await?;
    Ok(count.0 > 0)
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

pub async fn register_chat_member(
    pool: &SqlitePool,
    chat_id: i64,
    telegram_id: i64,
    username: Option<String>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO chat_members (chat_id, telegram_id, username)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(chat_id)
    .bind(telegram_id)
    .bind(username)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn unregister_chat_member(
    pool: &SqlitePool,
    chat_id: i64,
    telegram_id: i64,
) -> Result<()> {
    sqlx::query("DELETE FROM chat_members where chat_id = ? AND telegram_id = ?")
        .bind(chat_id)
        .bind(telegram_id)
        .execute(pool)
        .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn is_chat_member(pool: &SqlitePool, chat_id: i64, telegram_id: i64) -> Result<bool> {
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM chat_members WHERE chat_id = ? AND telegram_id = ?")
            .bind(chat_id)
            .bind(telegram_id)
            .fetch_one(pool)
            .await?;

    Ok(count.0 > 0)
}

pub async fn mark_film_as_seen(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    telegram_id: i64,
) -> Result<bool> {
    let result =
        sqlx::query("UPDATE films SET is_seen = ? WHERE kinopoisk_id = ? AND telegram_id = ?")
            .bind(true)
            .bind(kinopoisk_id)
            .bind(telegram_id)
            .execute(pool)
            .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn mark_film_unseen(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    telegram_id: i64,
) -> Result<bool> {
    let result =
        sqlx::query("UPDATE films set is_seen = ? WHERE kinopoisk_id = ? AND telegram_id = ?")
            .bind(false)
            .bind(kinopoisk_id)
            .bind(telegram_id)
            .execute(pool)
            .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_chat_films(pool: &SqlitePool, chat_id: i64) -> Result<Vec<Film>> {
    let films = sqlx::query_as::<_, Film>(
        r#"
            SELECT f.* FROM films f
            INNER JOIN chat_members cm on f.telegram_id == cm.telegram_id
            WHERE cm.chat_id == ? AND f.is_seen = 0
            ORDER BY f.created_at DESC
        "#,
    )
    .bind(chat_id)
    .fetch_all(pool)
    .await?;

    Ok(films)
}

pub async fn get_personal_films(pool: &SqlitePool, telegram_id: i64) -> Result<Vec<Film>> {
    let films = sqlx::query_as::<_, Film>(
        r#"
            SELECT f.* FROM films f
            WHERE f.telegram_id == ? AND f.is_seen = 0
            ORDER BY f.created_at DESC
        "#,
    )
    .bind(telegram_id)
    .fetch_all(pool)
    .await?;

    Ok(films)
}
