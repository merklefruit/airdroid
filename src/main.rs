#![allow(unused)] // TODO: remove for release

mod bot;
mod certstream;
mod error;
mod prelude;
mod utils;

use crate::{bot::Command, prelude::*};
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

    let handler = Update::filter_message().branch(
        dptree::entry()
            .filter_command::<Command>()
            .endpoint(bot::answer),
    );

    let mut dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![db.clone()])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build();

    // execute these tasks concurrently:
    join!(
        // 1. run the bot in the background to handle chat commands
        dispatcher.dispatch(),
        // 2. send automatic updates to the group chat when a new domain is found
        bot::send_update(
            bot,
            db.clone(),
            constants::GROUP_CHAT_ID,
            constants::UPDATE_INTERVAL
        ),
        // 3. scrape certstream for new domains and add them to the db
        certstream::scrape(db)
    );

    Ok(())
}
