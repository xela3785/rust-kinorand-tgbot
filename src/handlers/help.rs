use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use super::Command;

pub async fn handle(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let help_text = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, help_text).await?;
    Ok(())
}