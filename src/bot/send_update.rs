use crate::{certstream::dump, prelude::*};
use rocksdb::DB;
use std::{sync::Arc, time::Duration};
use teloxide::{prelude::*, types::Recipient};

pub async fn send_update(bot: Bot, db: Arc<DB>, chat_id: &str, update_interval: u64) -> Result<()> {
    loop {
        // 1. check if there are any new domains in rocksdb that match the tracked domains
        let new_domains = check_for_new_domains(db.clone())?;

        // 2. if there are, send a new message to the chat
        if let Some(new_domains) = new_domains {
            let text = new_domains.join(", ");
            bot.send_message(Recipient::from(chat_id.to_owned()), text)
                .await?;
        }

        tokio::time::sleep(Duration::from_secs(update_interval)).await;
    }

    Ok(())
}

fn check_for_new_domains(db: Arc<DB>) -> Result<Option<Vec<String>>> {
    // 1. get the list of tracked domains from the db
    // 2. get the list of all domains collected from the db
    // 3. check if there are any new domains that match the tracked keywords
    // 4. if there are, return them

    // 1.
    let tracked_keywords = db
        .get(constants::TRACKED_KEYWORDS_KEY)?
        .iter()
        .map(|domain| String::from_utf8(domain.to_vec()).unwrap())
        .collect::<Vec<_>>();

    // 2.
    let all_domains = dump(db)?;

    // 3.
    let new_domains = all_domains
        .iter()
        .filter(|domain| tracked_keywords.contains(domain.0))
        .collect::<Vec<_>>();

    if new_domains.is_empty() {
        return Ok(None);
    }

    // 4.
    Ok(Some(
        new_domains.iter().map(|domain| domain.0.clone()).collect(),
    ))
}
