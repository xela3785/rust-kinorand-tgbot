use anyhow::Result;
use sqlx::SqlitePool;

pub async fn mark_film_as_seen_for_user(
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

pub async fn mark_film_as_seen_for_chat(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    chat_id: i64,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE films SET is_seen = ? WHERE kinopoisk_id = ? AND telegram_id IN (
            SELECT telegram_id FROM chat_members WHERE chat_id = ?
        )
        "#,
    )
    .bind(true)
    .bind(kinopoisk_id)
    .bind(chat_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn mark_film_unseen_for_user(
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

pub async fn mark_film_unseen_for_chat(
    pool: &SqlitePool,
    kinopoisk_id: i64,
    chat_id: i64,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE films SET is_seen = ? WHERE kinopoisk_id = ? AND telegram_id IN (
            SELECT telegram_id FROM chat_members WHERE chat_id = ?
        )
        "#,
    )
    .bind(false)
    .bind(kinopoisk_id)
    .bind(chat_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
