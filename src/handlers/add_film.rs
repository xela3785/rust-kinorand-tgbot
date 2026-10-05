use sqlx::SqlitePool;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestLinkPreviewExt;
use teloxide::types::{InputFile, ParseMode};
use teloxide::utils::html::escape;

use crate::dialogue::{Dialogue, DialogueState, HandlerResult};
use crate::integrations::id_parser;
use crate::integrations::movie_api::KinoClient;
use crate::repository;

pub async fn handle(
    bot: Bot,
    msg: Message,
    pool: SqlitePool,
    api_client: KinoClient,
    dialogue: DialogueState,
    url: String,
) -> HandlerResult {
    let url = url.trim();

    if url.is_empty() {
        dialogue.update(Dialogue::WaitForUrl).await?;

        bot.send_message(
            msg.chat.id,
            "Укажи ссылку на фильм с кинопоиска в следующем сообщении. Например https://kinopoisk.ru/film/12345/"
        )
        .disable_link_preview(true)
        .await?;

        return Ok(());
    }

    process_url(bot, msg, pool, api_client, dialogue, url).await
}

pub async fn handle_url_message(
    bot: Bot,
    msg: Message,
    pool: SqlitePool,
    api_client: KinoClient,
    dialogue: DialogueState,
) -> HandlerResult {
    let url = match msg.text() {
        Some(text) => text.trim(),
        None => {
            bot.send_message(
                msg.chat.id,
                "🔴 Ожидалась ссылка на кинопоиск, пример: https://kinopoisk.ru/film/12345/. Попробуй еще раз или введи /cancel"
            )
            .disable_link_preview(true)
            .await?;
            return Ok(());
        }
    };

    process_url(bot, msg.clone(), pool, api_client, dialogue, url).await
}

async fn process_url(
    bot: Bot,
    msg: Message,
    pool: SqlitePool,
    api_client: KinoClient,
    dialogue: DialogueState,
    url: &str,
) -> HandlerResult {
    let kinopoisk_id = match id_parser::extract_id(url) {
        Ok(id) => id,
        Err(e) => {
            log::error!("Failed to extract ID from URL: {}", e);
            bot.send_message(msg.chat.id, "⚠️ Не удалось извлечь ID фильма из ссылки")
                .await?;
            return Ok(());
        }
    };

    if repository::film_exists(&pool, kinopoisk_id).await? {
        bot.send_message(msg.chat.id, "ℹ️ Такой фильм уже есть")
            .await?;
        dialogue.update(Dialogue::Start).await?;
        return Ok(());
    }

    let metadata = match api_client.find_by_id(kinopoisk_id, url).await {
        Ok(meta) => meta,
        Err(e) => {
            log::error!("Failed to fetch movie info: {}", e);
            bot.send_message(msg.chat.id, "⚠️ Не удалось получить информацию о фильме")
                .await?;
            dialogue.update(Dialogue::Start).await?;
            return Ok(());
        }
    };

    let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
    let username = msg.from.as_ref().and_then(|u| u.username.clone());

    match repository::add_film(&pool, telegram_id, username, &metadata).await {
        Ok(_) => {
            let title = metadata.title.unwrap();
            let original_title = metadata.original_title.unwrap();
            let year = metadata.year.unwrap();

            let movie_title = if original_title.is_empty() || title == original_title {
                format!("<b>{}</b>", escape(&title))
            } else {
                format!(
                    "<b>{}</b> <i>| {}</i>",
                    escape(&title),
                    escape(&original_title)
                )
            };

            let response = format!(
                "✨ <b>Фильм успешно добавлен!</b>\n\n\
                🎬 {}\n\
                📅 <code>{} год</code>",
                movie_title,
                escape(&year.to_string())
            );

            bot.send_photo(
                msg.chat.id,
                InputFile::url(metadata.poster_url.unwrap_or_default().parse().unwrap()),
            )
            .caption(response)
            .parse_mode(ParseMode::Html)
            .await?;
        }

        Err(e) => {
            log::error!("Failed to add film to database: {}", e);
            bot.send_message(msg.chat.id, "❌ Не удалось добавить фильм")
                .await?;
        }
    };

    dialogue.update(Dialogue::Start).await?;

    Ok(())
}
