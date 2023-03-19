use crate::prelude::*;
use rocksdb::DB;
use std::{sync::Arc, time::Duration};
use teloxide::{prelude::*, types::Recipient};

pub async fn send_update(bot: Bot, db: Arc<DB>, chat_id: &str, update_interval: u64) -> Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(update_interval)).await;

        // 1. check if there are any new domains in rocksdb that match the tracked domains
        // todo

        // 2. if there are, send a new message to the chat

        bot.send_message(Recipient::from(chat_id.to_owned()), "test")
            .await?;
    }

    Ok(())
}
