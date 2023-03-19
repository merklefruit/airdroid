use crate::{certstream::dump, prelude::*, utils};
use rocksdb::DB;
use std::{sync::Arc, time::Duration};
use teloxide::{prelude::*, types::Recipient};

pub async fn send_update(bot: Bot, db: Arc<DB>, chat_id: &str, update_interval: u64) -> Result<()> {
    loop {
        // 1. check if there are any new domains in rocksdb that match the tracked domains
        let new_domains = check_for_new_domains(db.clone())?;

        // 2. if there are, send a new message to the chat
        if let Some(new_domains) = new_domains {
            let text = f!(
                "Found new domain{}: {}",
                if new_domains.len() > 1 { "s" } else { "" },
                new_domains.join(", ")
            );

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

    let known_domains = utils::parse_csv_to_vec(db.get(constants::KNOWN_DOMAINS_KEY)?);
    log::debug!("Known domains: {:?}", known_domains);

    // 2.
    let all_domains = dump(db)?;
    log::debug!("All domains: {:?}", all_domains);

    // 3.
    let new_domains = all_domains
        .iter()
        .filter(|domain| known_domains.contains(domain.0))
        .collect::<Vec<_>>();

    log::debug!("New domains: {:?}", new_domains);

    if new_domains.is_empty() {
        return Ok(None);
    }

    // 4.
    Ok(Some(
        new_domains.iter().map(|domain| domain.0.clone()).collect(),
    ))
}
