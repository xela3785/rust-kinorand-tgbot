use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use teloxide::prelude::*;

mod handlers;
mod integrations;
mod models;
mod repository;

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
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:films.db?mode=rwc")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    log::info!("DB initialized successfully.");

    let api_client = KinoClient::new(api_key, base_url);

    let bot = Bot::new(token);

    log::info!("Bot started...");

    teloxide::dispatching::Dispatcher::builder(bot, handlers::schema())
        .dependencies(dptree::deps![pool, api_client])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
