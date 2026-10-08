use crate::dialogue::HandlerResult;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestLinkPreviewExt;
use teloxide::types::{Message, ParseMode};

pub async fn handle(bot: Bot, msg: Message) -> HandlerResult {
    let help_text = build_help_message();

    bot.send_message(msg.chat.id, help_text)
        .parse_mode(ParseMode::Html)
        .disable_link_preview(true)
        .await?;

    Ok(())
}

fn build_help_message() -> String {
    // Заголовок
    let mut text = String::from("🎬 <b>Кино трекер</b> — помощник для выбора фильмов\n\n");

    // Основная группа
    text.push_str("⚙️ <b>Основное</b>\n");
    text.push_str("  <code>/start</code> — начать работу с ботом\n");
    text.push_str("  <code>/help</code> — показать эту справку\n");
    text.push_str("  <code>/cancel</code> — отменить текущее действие\n\n");

    // Управление списком
    text.push_str("🎞 <b>Управление списком</b>\n");
    text.push_str("  <code>/list</code> — 📋 показать список фильмов\n");
    text.push_str("  <code>/add</code> <i>[ссылка на Кинопоиск]</i>\n");
    text.push_str("       ➕ добавить фильм в список\n");
    text.push_str("  <code>/delete</code> <i>[ссылка на Кинопоиск]</i>\n");
    text.push_str("       ❌ удалить фильм из списка\n");
    text.push_str("  <code>/count</code> — показать количество фильмов\n\n");

    // Просмотренные
    text.push_str("👁 <b>Просмотренные</b>\n");
    text.push_str("  <code>/seen</code> <i>[ссылка на Кинопоиск]</i>\n");
    text.push_str("       ✅ отметить фильм как просмотренный\n");
    text.push_str("  <code>/unseen</code> <i>[ссылка на Кинопоиск]</i>\n");
    text.push_str("       ↩️ вернуть фильм в список непросмотренных\n\n");

    // Выбор фильма
    text.push_str("🎲 <b>Выбор фильма</b>\n");
    text.push_str("  <code>/random</code> <i>[количество]</i>\n");
    text.push_str("       🎯 выбрать случайные фильмы\n");
    text.push_str("  <code>/random_per_user</code>\n");
    text.push_str("       👥 по одному фильму от каждого участника\n\n");

    // Чат
    text.push_str("💬 <b>Чат</b>\n");
    text.push_str("  <code>/join</code> — присоединиться к чату\n");
    text.push_str("  <code>/leave</code> — покинуть чат\n\n");

    // Подсказка
    text.push_str("💡 <i>Подсказка: ссылки можно отправлять прямо из Кинопоиска — бот распознает их автоматически.</i>");

    text
}
