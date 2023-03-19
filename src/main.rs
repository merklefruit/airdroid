#![allow(unused)] // TODO: remove for release

use crate::prelude::*;

mod certstream;
mod error;
mod prelude;
mod utils;

use std::time::Duration;
use teloxide::{prelude::*, types::Recipient, utils::command::BotCommands};
use tokio::join;

const GROUP_CHAT_ID: &'static str = "-900218105";
const SQLITE_DB_PATH: &'static str = "db.sqlite";
const ROCKSDB_PATH: &'static str = "certstream.db";
const UPDATE_INTERVAL: u64 = 10; // seconds

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

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    let bot_storage = x;

    join!(
        Command::repl(bot.clone(), answer), // Handle bot commands
        send_update(bot.clone()),           // Send updates to the chat autonomously
        certstream::scrape(ROCKSDB_PATH, bot_storage)  // Scrape CertStream for new domains
    );

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

async fn send_update(bot: Bot, bot_storage: BotStorage) -> Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(UPDATE_INTERVAL)).await;

        // 1. check if there are any new domains in rocksdb that match the tracked domains
        let domains_list = certstream::dump(ROCKSDB_PATH);

        if domains_list.is_empty() {
            continue;
        }

        // 2. if there are, send a new message to the chat

        bot.send_message(Recipient::from(GROUP_CHAT_ID.to_owned()), "test")
            .await?;
    }

    Ok(())
}
