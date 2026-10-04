use teloxide::prelude::*;

pub async fn handle(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(
        msg.chat.id,
        "👋 Привет!\n\nЯ бот, который рандомно выбирает фильмы в чате, которые добавили пользователи, чтобы вам было что посмотреть в компании!\nНапиши `/help`, чтобы посмотреть доступные команды."
    )
        .await?;
    Ok(())
}
