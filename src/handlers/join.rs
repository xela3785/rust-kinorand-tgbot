use sqlx::SqlitePool;
use teloxide::prelude::*;

use crate::{dialogue::HandlerResult, repository};

pub async fn handle(bot: Bot, msg: Message, pool: SqlitePool) -> HandlerResult {
    if msg.chat.is_private() {
        bot.send_message(
            msg.chat.id,
            "Вы не можете присоединиться к чату в личном сообщении",
        )
        .await?;
        return Ok(());
    }

    let chat_id = msg.chat.id.0;
    let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
    let username = msg.from.as_ref().and_then(|u| u.username.clone());

    repository::register_chat_member(&pool, chat_id, telegram_id, username).await?;

    bot.send_message(
        msg.chat.id,
        "Вы успешно присоединились к чату. Теперь добавленные вами фильмы будут учавствовать в выборе случайных",
    )
        .await?;

    Ok(())
}
