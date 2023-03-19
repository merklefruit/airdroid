#![allow(unused)] // TODO: remove for release

mod bot;
mod certstream;
mod error;
mod prelude;

use crate::prelude::*;
use rocksdb::{Options, DB};
use std::{sync::Arc, time::Duration};
use teloxide::prelude::*;
use tokio::join;

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
    let db = Arc::new(DB::open(&options, constants::ROCKSDB_PATH).unwrap());

    let mut keywords_to_track = db
        .get(constants::TRACKED_KEYWORDS_KEY)?
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .unwrap_or_default()
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // execute these tasks concurrently:
    join!(
        // 1. run the bot in the background to handle chat commands
        bot::Command::repl(bot.clone(), bot::answer),
        // 2. send automatic updates to the group chat when a new domain is found
        bot::send_update(
            bot.clone(),
            db.clone(),
            constants::GROUP_CHAT_ID,
            constants::UPDATE_INTERVAL
        ),
        // 3. scrape certstream for new domains and add them to the db
        certstream::scrape(db, &keywords_to_track)
    );

    Ok(())
}
