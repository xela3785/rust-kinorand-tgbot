use sqlx::SqlitePool;
use teloxide::{prelude::*, sugar::request::RequestLinkPreviewExt};

use crate::{dialogue::HandlerResult, integrations::id_parser, repository};

pub async fn handle(bot: Bot, msg: Message, pool: SqlitePool, url: String) -> HandlerResult {
    let url = url.trim();

    if url.is_empty() {
        bot.send_message(
            msg.chat.id,
            "⚠️ Укажи ссылку на фильм с кинопоиска. Пример `/unseen https://kinopoisk.ru/film/12345/`"
        )
        .disable_link_preview(true)
        .await?;
        return Ok(());
    }

    let kinopoisk_id = match id_parser::extract_id(url) {
        Ok(id) => id,
        Err(e) => {
            log::error!("Error parsing id from url: {}", e);
            bot.send_message(msg.chat.id, "⚠️ Не удалось получить id фильма из ссылки")
                .await?;
            return Ok(());
        }
    };

    let updated = if msg.chat.is_private() {
        let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

        if !repository::film_exists_for_user(&pool, kinopoisk_id, telegram_id).await? {
            bot.send_message(msg.chat.id, "❌ Фильм не найден в базе данных")
                .await?;
            return Ok(());
        }

        repository::mark_film_unseen_for_user(&pool, kinopoisk_id, telegram_id).await?
    } else {
        let chat_id = msg.chat.id.0;

        if !repository::film_exists_for_chat(&pool, kinopoisk_id, chat_id).await? {
            bot.send_message(msg.chat.id, "❌ Фильм не найден в базе данных")
                .await?;
            return Ok(());
        }

        repository::mark_film_unseen_for_chat(&pool, kinopoisk_id, chat_id).await?
    };

    if updated {
        bot.send_message(msg.chat.id, "✅ Фильм успешно отмечен как непросмотренный")
            .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            "⚠️ Не удалось отметить фильм как непросмотренный",
        )
        .await?;
    }
    Ok(())
}
