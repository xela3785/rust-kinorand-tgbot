use teloxide::prelude::*;
use crate::dialogue::HandlerResult;

pub async fn handle(
    bot: Bot,
    msg: Message,
) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "👋 Привет!\n\nЯ бот, который рандомно выбирает фильмы в чате, которые добавили пользователи, чтобы вам было что посмотреть в компании!\nНапиши `/help`, чтобы посмотреть доступные команды."
    )
        .await?;
    Ok(())
}
