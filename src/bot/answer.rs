use super::Command;
use crate::prelude::*;
use rocksdb::DB;
use std::sync::Arc;
use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn answer(db: Arc<DB>, bot: Bot, msg: Message, cmd: Command) -> Result<()> {
    match cmd {
        // Send a message with the list of commands.
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }

        // Send a message with the list of currently tracked keywords.
        Command::Domains => {
            bot.send_message(
                msg.chat.id,
                db.get(constants::TRACKED_KEYWORDS_KEY)?
                    .map(|x| String::from_utf8(x.to_vec()).unwrap())
                    .unwrap_or_else(|| "No domains are currently tracked.".to_string()),
            )
            .await?
        }

        // Add a keyword to the list of tracked keywords.
        Command::Track(domain) => {
            let mut domains = db.get(constants::TRACKED_KEYWORDS_KEY)?.unwrap_or_default();

            // Add a comma if there are already domains in the list
            let should_add_comma = if !domains.is_empty() { "," } else { "" };
            domains.extend_from_slice(f!("{}{}", should_add_comma, domain).as_bytes());
            db.put(constants::TRACKED_KEYWORDS_KEY, domains)?;

            bot.send_message(msg.chat.id, "Added domain to tracking list.")
                .await?
        }

        // Send a message with the current chat id.
        Command::ChatId => bot.send_message(msg.chat.id, f!("{}", msg.chat.id)).await?,
    };

    Ok(())
}
