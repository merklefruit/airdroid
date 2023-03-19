use std::sync::Arc;

use super::Command;
use crate::prelude::*;
use rocksdb::DB;
use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Domains => bot.send_message(msg.chat.id, "test".to_string()).await?,
        Command::ChatId => bot.send_message(msg.chat.id, f!("{}", msg.chat.id)).await?,
    };

    Ok(())
}
