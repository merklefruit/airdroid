mod bot;
mod certstream;
mod error;
mod prelude;
mod utils;

use crate::{bot::Command, prelude::*};
use rocksdb::{Options, DB};
use std::sync::Arc;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    log::info!("Starting bot...");
    run().await?;
    log::info!("Bot stopped.");

    Ok(())
}

async fn run() -> Result<()> {
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
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build();

    tokio::spawn(bot::send_updates(bot, db.clone()));
    tokio::spawn(certstream::scrape(db.clone()));

    dispatcher.dispatch().await;

    Ok(())
}
