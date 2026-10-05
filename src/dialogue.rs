use serde::{Deserialize, Serialize};
use teloxide::dispatching::dialogue::InMemStorage;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Dialogue {
    #[default]
    Start,

    // Waiting for KP kink
    WaitForUrl,
}

pub type DialogueState =
    teloxide::dispatching::dialogue::Dialogue<Dialogue, InMemStorage<Dialogue>>;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
