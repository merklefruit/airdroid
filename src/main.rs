#![allow(unused)] // TODO: remove for release

mod bot;
mod certstream;
mod error;
mod prelude;
mod utils;

use crate::prelude::*;
use rocksdb::{Options, DB};
use std::{sync::Arc, time::Duration};
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
pub enum Command {
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

    let mut options = Options::default();
    options.set_error_if_exists(false);
    options.create_if_missing(true);
    options.create_missing_column_families(true);

    let bot = Bot::from_env();
    let db = Arc::new(DB::open(&options, ROCKSDB_PATH).unwrap());

    // todo: make the initial list a db key-value pair and get it from there
    let mut keywords_to_track = vec!["arbitrum", "zksync", "airdrop"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    join!(
        Command::repl(bot.clone(), bot::answer),
        bot::send_update(bot.clone(), db.clone(), GROUP_CHAT_ID, UPDATE_INTERVAL),
        certstream::scrape(db.clone(), &keywords_to_track)
    );

    Ok(())
}
