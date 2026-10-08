use anyhow::Result;
use sqlx::SqlitePool;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestLinkPreviewExt;
use teloxide::types::ParseMode;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::html::escape;

use crate::{dialogue::HandlerResult, repository};

const PAGE_SIZE: i64 = 5;

#[derive(Debug, Clone)]
struct RenderedList {
    text: String,
    keyboard: Option<InlineKeyboardMarkup>,
}

pub async fn handle(bot: Bot, msg: Message, pool: SqlitePool) -> HandlerResult {
    let Some(user) = msg.from else {
        bot.send_message(msg.chat.id, "Не получилось определить пользователя")
            .await?;
        return Ok(());
    };

    let telegram_id = user.id.0 as i64;
    let rendered = build_user_films_page(&pool, telegram_id, 0).await?;

    let mut req = bot
        .send_message(msg.chat.id, rendered.text)
        .parse_mode(ParseMode::Html)
        .disable_link_preview(true);

    if let Some(keyboard) = rendered.keyboard {
        req = req.reply_markup(keyboard);
    }

    req.await?;
    Ok(())
}

pub async fn handle_list_callback(bot: Bot, q: CallbackQuery, pool: SqlitePool) -> HandlerResult {
    let callback_id = q.id.clone();

    let Some(data) = q.data else {
        bot.answer_callback_query(callback_id).await?;
        return Ok(());
    };

    if data == "noop" {
        bot.answer_callback_query(callback_id).await?;
        return Ok(());
    }

    let Some((owner_id, requested_page)) = parse_list_callback(&data) else {
        bot.answer_callback_query(callback_id).await?;
        return Ok(());
    };

    let from_id = q.from.id.0 as i64;

    if from_id != owner_id {
        bot.answer_callback_query(callback_id)
            .text("Это список другого пользователя")
            .show_alert(true)
            .await?;
        return Ok(());
    }

    let Some(msg) = q.message else {
        bot.answer_callback_query(callback_id).await?;
        return Ok(());
    };

    let chat_id = msg.chat().id;
    let message_id = msg.id();

    let rendered = build_user_films_page(&pool, owner_id, requested_page).await?;

    let text = rendered.text.clone();
    let keyboard = rendered.keyboard.clone();

    let mut req = bot
        .edit_message_text(chat_id, message_id, text.clone())
        .parse_mode(ParseMode::Html)
        .disable_link_preview(true);

    req = match keyboard.clone() {
        Some(kb) => req.reply_markup(kb),
        None => req.reply_markup(InlineKeyboardMarkup::new(
            Vec::<Vec<InlineKeyboardButton>>::new(),
        )),
    };

    match req.await {
        Ok(_) => {
            bot.answer_callback_query(callback_id).await?;
            return Ok(());
        }
        Err(e) => {
            log::warn!("Error editing message: {}", e);

            let mut req = bot.send_message(chat_id, text);

            if let Some(kb) = keyboard {
                req = req.reply_markup(kb);
            }

            req.await?;

            bot.answer_callback_query(callback_id)
                .text("Открыл список в новом сообщении")
                .await?;
        }
    }

    Ok(())
}

async fn build_user_films_page(
    pool: &SqlitePool,
    telegram_id: i64,
    raw_page: i64,
) -> Result<RenderedList> {
    let total = repository::count_user_films(pool, telegram_id).await?;

    if total == 0 {
        return Ok(RenderedList {
            text: concat!(
                "📋 У тебя пока нет фильмов в списке. \n",
                "Добавь фильм командой /add"
            )
            .to_string(),
            keyboard: None,
        });
    }

    let pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
    let page = raw_page.clamp(0, pages.saturating_sub(1));
    let offset = page * PAGE_SIZE;

    let films = repository::get_user_films_list(pool, telegram_id, PAGE_SIZE, offset).await?;

    let mut text = format!(
        "📋 Твой список фильмов - стр. {}/{} (всего {})\nСтатус просмотра: 🟢 - Просмотрен, 🔴 - Не просмотрен\n\n",
        page + 1,
        pages,
        total,
    );

    for (idx, film) in films.iter().enumerate() {
        let num = offset + idx as i64 + 1;

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

        let seen_mark = if film.is_seen { "🟢" } else { "🔴" };

        text.push_str(&format!(
            "{}. <a href=\"{}\">{}</a> - {} - {}\n\n",
            num,
            film.kinopoisk_url.as_deref().unwrap(),
            movie_title,
            year_str,
            seen_mark
        ));
    }

    let keyboard = build_pagination_keyboard(telegram_id, page, pages);

    Ok(RenderedList { text, keyboard })
}

fn parse_list_callback(data: &str) -> Option<(i64, i64)> {
    let mut parts = data.split(':');

    if parts.next()? != "list" {
        return None;
    }

    let owner_id = parts.next()?.parse().ok()?;
    let page = parts.next()?.parse().ok()?;

    Some((owner_id, page))
}

fn build_pagination_keyboard(owner_id: i64, page: i64, pages: i64) -> Option<InlineKeyboardMarkup> {
    if pages <= 1 {
        return None;
    }

    let mut row: Vec<InlineKeyboardButton> = Vec::new();

    if page > 0 {
        if page > 1 {
            row.push(InlineKeyboardButton::callback(
                "⏮️",
                format!("list:{}:0", owner_id),
            ));
        }

        row.push(InlineKeyboardButton::callback(
            "◀️",
            format!("list:{}:{}", owner_id, page - 1),
        ));
    }

    row.push(InlineKeyboardButton::callback(
        format!("{}/{}", page + 1, pages),
        "noop".to_string(),
    ));

    if page + 1 < pages {
        row.push(InlineKeyboardButton::callback(
            "▶️",
            format!("list:{}:{}", owner_id, page + 1),
        ));

        if page + 2 < pages {
            row.push(InlineKeyboardButton::callback(
                "⏭️",
                format!("list:{}:{}", owner_id, pages - 1),
            ))
        }
    }

    Some(InlineKeyboardMarkup::new(vec![row]))
}
