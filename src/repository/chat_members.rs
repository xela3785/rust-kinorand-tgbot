use anyhow::Result;
use sqlx::SqlitePool;

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
