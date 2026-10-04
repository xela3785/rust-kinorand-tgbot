use teloxide::prelude::*;

pub async fn handle(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(
        msg.chat.id,
        "👋 Привет!\n\nЯ бот, который ищет говно среди алмазов для того, чтобы вам было что посмотреть в компании!\nНапиши `/help`, чтобы посмотреть доступные команды."
    )
        .await?;
    Ok(())
}
