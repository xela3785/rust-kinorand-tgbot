use sqlx::SqlitePool;
use std::collections::HashMap;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestLinkPreviewExt;
use teloxide::types::ParseMode;
use teloxide::utils::html::escape;

use crate::{dialogue::HandlerResult, repository};



pub async fn handle(
    bot: Bot,
    msg: Message,
    pool: SqlitePool,
) -> HandlerResult {
    let chat_id = msg.chat.id.0;

    if msg.chat.is_private() {
        bot.send_message(msg.chat.id, "⚠️ Эта команда не доступна в личном чате.")
            .await?;
        return Ok(());
    }

    let films = repository::get_chat_films(&pool, chat_id).await?;

    if films.is_empty() {
        bot.send_message(msg.chat.id, "😒 В этом чате нет добавленных фильмов")
            .await?;
        return Ok(());
    }

    let mut user_films: HashMap<i64, Vec<_>> = HashMap::new();
    for film in films {
        user_films.entry(film.telegram_id).or_default().push(film);
    }

    let mut selected = Vec::new();
    for (_user_id, films_list) in &user_films {
        let idx = fastrand::usize(..films_list.len());
        selected.push(films_list[idx].clone());
    }

    if selected.is_empty() {
        bot.send_message(msg.chat.id, "🎲 Не удалось выбрать фильмы.")
            .await?;
        return Ok(());
    }

    let mut response = String::from("🎲 Кажется нащупал! Выбранные фильмы:\n\n");
    for film in selected {
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
