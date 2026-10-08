use anyhow::Result;
use sqlx::{FromRow, SqlitePool};

use crate::models::Film;

#[derive(Debug, FromRow)]
pub struct UserFilmPageRow {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub year: Option<i64>,
    pub is_seen: bool,
    pub kinopoisk_url: Option<String>,
}

pub async fn get_user_films_list(
    pool: &SqlitePool,
    telegram_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<UserFilmPageRow>> {
    let films = sqlx::query_as::<_, UserFilmPageRow>(
        r#"
        SELECT title, original_title, year, kinopoisk_id, is_seen, kinopoisk_url
        FROM films
        WHERE telegram_id = ?
        ORDER BY is_seen ASC, created_at DESC, id DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(telegram_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(films)
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
