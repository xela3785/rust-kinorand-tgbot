use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

mod dialogue;
mod handlers;
mod integrations;
mod models;
mod repository;

use dialogue::Dialogue;
use integrations::movie_api::KinoClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    pretty_env_logger::init();

    let token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN environment variable not set");

    let api_key = env::var("KINO_API_KEY").expect("KINO_API_KEY environment variable not set");
    let base_url =
        env::var("KINO_API_BASE_URL").expect("KINO_API_BASE_URL environment variable not set");

    log::info!("DB initialized...");

    let db_path =
        std::env::var("DATABASE_PATH").unwrap_or_else(|_| "/app/data/films.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{}?mode=rwc", db_path))
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    log::info!("DB initialized successfully.");

    let api_client = KinoClient::new(api_key, base_url);

    let bot = Bot::new(token);

    if let Err(err) = bot.set_my_commands(handlers::Command::bot_commands()).await {
        log::error!("Не удалось установить меню комманд: {:?}", err);
    }

    let dialogue_storage = InMemStorage::<Dialogue>::new();

    log::info!("Bot started...");

    teloxide::dispatching::Dispatcher::builder(bot, handlers::schema())
        .dependencies(dptree::deps![pool, api_client, dialogue_storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
