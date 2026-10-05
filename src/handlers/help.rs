use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use super::Command;
use crate::dialogue::HandlerResult;

pub async fn handle(
    bot: Bot,
    msg: Message,
) -> HandlerResult {
    let help_text = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, help_text).await?;
    Ok(())
}
