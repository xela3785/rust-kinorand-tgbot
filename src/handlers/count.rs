use sqlx::SqlitePool;
use teloxide::prelude::*;

use crate::{dialogue::HandlerResult, repository};

pub async fn handle(bot: Bot, msg: Message, pool: SqlitePool) -> HandlerResult {
    let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    if msg.chat.is_private() {
        let count = repository::count_user_films(&pool, telegram_id).await?;

        bot.send_message(msg.chat.id, format!("📊 Тобой добавлено {} фильмов", count))
            .await?;
    } else {
        let chat_id = msg.chat.id.0;
        let count = repository::count_chat_films(&pool, chat_id).await?;

        bot.send_message(
            msg.chat.id,
            format!("📊 В этом чате добавлено {} фильмов", count),
        )
        .await?;
    }

    Ok(())
}
