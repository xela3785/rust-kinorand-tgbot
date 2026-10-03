use teloxide::prelude::*;
use sqlx::SqlitePool;

use crate::integrations::id_parser;
use crate::integrations::movie_api::KinoClient;
use crate::repository;

pub async fn handle(
    bot: Bot,
    msg: Message,
    pool: SqlitePool,
    api_client: KinoClient,
    url: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = url.trim();

    if url.is_empty() {
        bot.send_message(
            msg.chat.id,
            "⚠️ Укажи ссылку на фильм с кинопоиска. Пример: \n `/add https://kinopoisk.ru/film/12345/`"
        ).await?;
        return Ok(());
    }

    let kinopoisk_id = match id_parser::extract_id(url) {
        Ok(id) => id,
        Err(e) => {
            log::error!("Failed to extract ID from URL: {}", e);
            bot.send_message(
                msg.chat.id,
                "⚠️ Не удалось извлечь ID фильма из ссылки"
            ).await?;
            return Ok(());
        }
    };

    if repository::film_exists(&pool, kinopoisk_id).await? {
        bot.send_message(
            msg.chat.id,
            "ℹ️ Такой фильм уже есть"
        ).await?;
        return Ok(());
    }

    let metadata = match api_client.find_by_id(kinopoisk_id, url).await {
        Ok(meta) => meta,
        Err(e) => {
            log::error!("Failed to fetch movie info: {}", e);
            bot.send_message(
                msg.chat.id,
                "⚠️ Не удалось получить информацию о фильме"
            ).await?;
            return Ok(());
        }
    };

    let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
    let username = msg.from.as_ref().and_then(|u| u.username.clone());

    match repository::add_film(&pool, telegram_id, username, &metadata).await {
        Ok(_) => {
            let response = format!(
                "✅ Фильм добавлен! 🎬\n{} ({})\n{}",
                metadata.title.unwrap_or_default(),
                metadata.original_title.unwrap_or_default(),
                metadata.year.unwrap_or_default(),
            );

            bot.send_message(
                msg.chat.id,
                response
            ).await?;
        }

        Err(e) => {
            log::error!("Failed to add film to database: {}", e);
            bot.send_message(
                msg.chat.id,
                "❌ Не удалось добавить фильм"
            ).await?;
        }
    }

    Ok(())
}