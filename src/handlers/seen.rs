use sqlx::SqlitePool;
use teloxide::prelude::*;

use crate::{dialogue::HandlerResult, integrations::id_parser, repository};

pub async fn handle(bot: Bot, msg: Message, pool: SqlitePool, url: String) -> HandlerResult {
    let url = url.trim();

    if url.is_empty() {
        bot.send_message(
            msg.chat.id,
            "⚠️ Укажи ссылку на фильм с кинопоиска. Пример: \n `/seen https://kinopoisk.ru/film/12345/`"
        ).await?;
        return Ok(());
    }

    let kinopoisk_id = match id_parser::extract_id(url) {
        Ok(id) => id,
        Err(e) => {
            log::error!("Failed extract id from url: {}", e);
            bot.send_message(msg.chat.id, "⚠️ Не удалось извлечь ID фильма из ссылки.")
                .await?;
            return Ok(());
        }
    };

    if !repository::film_exists(&pool, kinopoisk_id).await? {
        bot.send_message(msg.chat.id, "Не найдено фильма в базе данных")
            .await?;
        return Ok(());
    }

    let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    let updated = repository::mark_film_as_seen(&pool, kinopoisk_id, telegram_id).await?;

    if updated {
        bot.send_message(msg.chat.id, "✅ Фильм успешно отмечен как просмотренный")
            .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            "⚠️ Не удалось отметить фильм как просмотренный",
        )
        .await?;
    }

    Ok(())
}
