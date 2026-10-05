use sqlx::SqlitePool;
use teloxide::prelude::*;

use crate::{dialogue::HandlerResult, repository};

pub async fn handle(bot: Bot, msg: Message, pool: SqlitePool) -> HandlerResult {
    if msg.chat.is_private() {
        bot.send_message(msg.chat.id, "Вы не можете выйти из чата в личном сообщении")
            .await?;
        return Ok(());
    }

    let chat_id = msg.chat.id.0;
    let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    repository::unregister_chat_member(&pool, chat_id, telegram_id).await?;

    bot.send_message(msg.chat.id, "✅ Вы успешно вышли из чата")
        .await?;

    Ok(())
}
