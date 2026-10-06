use sqlx::SqlitePool;
use teloxide::dispatching::UpdateHandler;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::dialogue::{Dialogue, DialogueState, HandlerResult};
use crate::integrations::movie_api::KinoClient;
use crate::repository;

mod add_film;
mod count;
mod delete;
mod help;
mod join;
mod leave;
mod random;
mod random_per_user;
mod seen;
mod start;
mod unseen;

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Начать работу с ботом")]
    Start,

    #[command(description = "Показать справку")]
    Help,

    #[command(
        description = "Добавить фильм: /add [ссылка на кинопоиск]",
        parse_with = "split"
    )]
    Add(String),

    #[command(description = "Показать количество фильмов")]
    Count,

    #[command(description = "Выбрать случайные n фильмов: /random [количество]")]
    Random(i32),

    #[command(
        description = "Выбрать по фильму от каждого участника",
        rename = "random_per_user"
    )]
    RandomPerUser,

    #[command(description = "Отметить просмотренным: /seen [ссылка на кинопоиск]")]
    Seen(String),

    #[command(description = "Отметить непросмотренным: /unseen [ссылка на кинопоиск]")]
    Unseen(String),

    #[command(description = "Удалить фильм: /delete [ссылка на кинопоиск]")]
    Delete(String),

    #[command(description = "Присоединится к чату")]
    Join,

    #[command(description = "Покинуть чат")]
    Leave,

    #[command(description = "Отменить текущее действие")]
    Cancel,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    // use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .enter_dialogue::<Message, InMemStorage<Dialogue>, Dialogue>()
        .endpoint(handle_command);

    let message_handler = Update::filter_message().branch(command_handler).branch(
        dptree::entry()
            .enter_dialogue::<Message, InMemStorage<Dialogue>, Dialogue>()
            .endpoint(handle_message),
    );

    message_handler
}

async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    pool: SqlitePool,
    api_client: KinoClient,
    dialogue: DialogueState,
) -> HandlerResult {
    if !msg.chat.is_private() {
        if let Some(ref user) = msg.from {
            let chat_id = msg.chat.id.0;
            let telegram_id = user.id.0 as i64;
            let username = user.username.clone();

            repository::register_chat_member(&pool, chat_id, telegram_id, username).await?;
        }
    }

    dialogue.update(Dialogue::Start).await?;

    match cmd {
        Command::Start => {
            start::handle(bot, msg).await?;
        }
        Command::Help => {
            help::handle(bot, msg).await?;
        }
        Command::Add(url) => {
            add_film::handle(bot, msg, pool, api_client, dialogue, url).await?;
        }
        Command::Count => {
            count::handle(bot, msg, pool).await?;
        }
        Command::Random(count) => {
            random::handle(bot, msg, pool, count).await?;
        }
        Command::RandomPerUser => {
            random_per_user::handle(bot, msg, pool).await?;
        }
        Command::Seen(url) => {
            seen::handle(bot, msg, pool, url).await?;
        }
        Command::Unseen(url) => {
            unseen::handle(bot, msg, pool, url).await?;
        }
        Command::Delete(url) => {
            delete::handle(bot, msg, pool, url).await?;
        }
        Command::Join => {
            join::handle(bot, msg, pool).await?;
        }
        Command::Leave => {
            leave::handle(bot, msg, pool).await?;
        }
        Command::Cancel => {
            bot.send_message(msg.chat.id, "✅ Операция отменена.")
                .await?;
        }
    }
    Ok(())
}

async fn handle_message(
    bot: Bot,
    msg: Message,
    dialogue: DialogueState,
    state: Dialogue,
    pool: SqlitePool,
    api_client: KinoClient,
) -> HandlerResult {
    if !msg.chat.is_private() {
        if let Some(ref user) = msg.from {
            let chat_id = msg.chat.id.0;
            let telegram_id = user.id.0 as i64;
            let username = user.username.clone();

            repository::register_chat_member(&pool, chat_id, telegram_id, username).await?;
        }
    }

    match state {
        Dialogue::Start => {
            bot.send_message(
                msg.chat.id,
                "Неизвестная команда. Напиши /help чтобы узнать список доступных комманд.",
            )
            .await?;
        }
        Dialogue::WaitForUrl => {
            add_film::handle_url_message(bot, msg, pool, api_client, dialogue).await?;
        }
    }

    Ok(())
}
