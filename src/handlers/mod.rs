use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::dispatching::UpdateHandler;
use sqlx::SqlitePool;

use crate::integrations::movie_api::KinoClient;

mod start;
mod help;
mod add_film;


#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Начать работу с ботом")]
    Start,

    #[command(description = "Показать справку")]
    Help,

    #[command(description = "Добавить жемчужину: /add <url>")]
    Add(String),
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    // use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .endpoint(handle_command);

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(|msg: Message, bot: Bot| async move {
            bot.send_message(msg.chat.id, "Неизвестная комманда. /help для справки")
                .await?;
            Ok(())
        }));

    message_handler
}

async fn handle_command(
    bot: Bot,
    msg: Message, 
    cmd: Command,
    pool: SqlitePool,
    api_client: KinoClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    match cmd {
        Command::Start => {
            start::handle(bot, msg).await?;
        }
        Command::Help => {
            help::handle(bot, msg).await?;
        }
        Command::Add(url) => {
            add_film::handle(bot, msg, pool, api_client, url).await?;
        }
    }
    Ok(())
}