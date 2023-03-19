#![allow(unused)] // TODO: remove for release

mod bot;
mod certstream;
mod error;
mod prelude;
mod utils;

use crate::prelude::*;
use rocksdb::{Options, DB};
use std::{sync::Arc, time::Duration};
use teloxide::prelude::*;
use tokio::join;

const GROUP_CHAT_ID: &'static str = "-900218105";
const ROCKSDB_PATH: &'static str = "certstream.db";
const UPDATE_INTERVAL: u64 = 10; // seconds

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

    // execute these tasks concurrently:
    join!(
        // 1. run the bot in the background to handle chat commands
        bot::Command::repl(bot.clone(), bot::answer),
        // 2. send automatic updates to the group chat when a new domain is found
        bot::send_update(bot.clone(), db.clone(), GROUP_CHAT_ID, UPDATE_INTERVAL),
        // 3. scrape certstream for new domains and add them to the db
        certstream::scrape(db.clone(), &keywords_to_track)
    );

    Ok(())
}
