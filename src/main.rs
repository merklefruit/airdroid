#![allow(unused)] // TODO: remove for release

use crate::prelude::*;

mod error;
mod prelude;
mod utils;

use std::time::Duration;
use teloxide::{
    dispatching::dialogue::{serializer::Json, ErasedStorage, SqliteStorage, Storage},
    prelude::*,
    types::Recipient,
    utils::command::BotCommands,
};
use tokio::join;

const GROUP_CHAT_ID: &str = "-900218105";

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "list tracked domain names.")]
    Domains,
    #[command(description = "get the current chat id.")]
    ChatId,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
enum State {
    #[default]
    Domains,
}

type BotStorage = std::sync::Arc<ErasedStorage<State>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    let bot_storage: BotStorage = SqliteStorage::open("db.sqlite", Json).await?.erase();

    join!(Command::repl(bot.clone(), answer), send_update(bot.clone()));

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Domains => bot.send_message(msg.chat.id, f!("test")).await?,
        Command::ChatId => bot.send_message(msg.chat.id, f!("{}", msg.chat.id)).await?,
    };

    Ok(())
}

async fn send_update(bot: Bot) -> Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        // 1. check if there are any new domains in rocksdb that match the tracked domains
        // 2. if there are, send a new message to the chat

        bot.send_message(Recipient::from(GROUP_CHAT_ID.to_owned()), "test")
            .await?;
    }

    Ok(())
}
