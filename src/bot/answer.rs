use super::Command;
use crate::prelude::*;
use rocksdb::DB;
use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn answer(db: Arc<DB>, bot: Bot, msg: Message, cmd: Command) -> Result<()> {
    match cmd {
        // Send a message with the list of commands.
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }

        // Send a message with the list of currently tracked keywords.
        Command::Keywords => {
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

            domains.extend_from_slice(
                f!("{}{}", if !domains.is_empty() { "," } else { "" }, domain).as_bytes(),
            );

            db.put(constants::TRACKED_KEYWORDS_KEY, domains)?;

            bot.send_message(
                msg.chat.id,
                f!("Added \"{domain}\" to the tracked keywords."),
            )
            .await?
        }

        // Send a message with the current chat id.
        Command::ChatId => bot.send_message(msg.chat.id, f!("{}", msg.chat.id)).await?,
    };

    Ok(())
}
