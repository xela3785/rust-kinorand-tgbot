use sqlx::SqlitePool;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestLinkPreviewExt;
use teloxide::types::ParseMode;
use teloxide::utils::html::escape;

use crate::{dialogue::HandlerResult, repository};

fn choose_multiple<T: Clone>(items: &[T], count: usize) -> Vec<T> {
    let mut indices: Vec<usize> = (0..items.len()).collect();
    let take = count.min(items.len());
    let mut result = Vec::with_capacity(take);

    for _ in 0..take {
        let idx = fastrand::usize(..indices.len());
        result.push(items[indices[idx]].clone());
        indices.swap_remove(idx);
    }

    result
}

pub async fn handle(bot: Bot, msg: Message, pool: SqlitePool, count: i32) -> HandlerResult {
    let chat_id = msg.chat.id.0;

    if msg.chat.is_private() {
        bot.send_message(
            msg.chat.id,
            "⚠️ Эта команда не доступна в личном чате (Пока что).",
        )
        .await?;
        return Ok(());
    }

    if count < 1 {
        bot.send_message(msg.chat.id, "⚠️ Количество фильмов должно быть больше 0.")
            .await?;
        return Ok(());
    }

    let films = repository::get_chat_films(&pool, chat_id).await?;

    if films.is_empty() {
        bot.send_message(msg.chat.id, "😒 В этом чате нет добавленных фильмов")
            .await?;
        return Ok(());
    }

    let selected_films = choose_multiple(&films, count as usize);

    let mut response = String::from("🎲 Кажется нащупал! Выбранные фильмы:\n\n");
    for film in selected_films {
        let user_display = film
            .username
            .map(|u| format!("@{}", u))
            .unwrap_or_else(|| format!("ID: {}", film.telegram_id));

        let movie_title = if film.original_title.is_none() || film.title == film.original_title {
            format!("<b>{}</b>", escape(film.title.as_deref().unwrap()))
        } else {
            format!(
                "<b>{}</b> <i>| {}</i>",
                escape(film.title.as_deref().unwrap()),
                escape(film.original_title.as_deref().unwrap())
            )
        };

        let year_str = film.year.map(|y| format!("{} год", y)).unwrap_or_default();

        response.push_str(&format!(
            " 🎗️ {}\n 🎥 <a href=\"{}\">{}</a> - {}\n\n ",
            user_display, film.kinopoisk_url, movie_title, year_str
        ));
    }

    bot.send_message(msg.chat.id, response)
        .parse_mode(ParseMode::Html)
        .disable_link_preview(true)
        .await?;

    Ok(())
}
