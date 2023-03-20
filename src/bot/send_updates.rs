use crate::{certstream, prelude::*, utils};
use rocksdb::DB;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use teloxide::{prelude::*, types::Recipient};

pub async fn send_updates(bot: Bot, db: Arc<DB>) -> Result<()> {
    loop {
        log::debug!("Checking for new domains...");

        if let Some(new_domains) = check_for_new_domains(&db)? {
            let text = f!(
                "Found new domain{}: {}",
                if new_domains.len() > 1 { "s" } else { "" },
                new_domains.join(", ")
            );

            log::debug!("Sending message: {}", text);
            // todo: add ability to send to multiple chats via a reserved key-value pair
            bot.send_message(Recipient::from(UserId(constants::GROUP_CHAT_ID)), text)
                .await?;
        } else {
            log::debug!(
                "No new domains found, retrying in {}s",
                constants::UPDATE_INTERVAL
            );
        }

        tokio::time::sleep(Duration::from_secs(constants::UPDATE_INTERVAL)).await;
    }
}

fn check_for_new_domains(db: &Arc<DB>) -> Result<Option<Vec<String>>> {
    let known_domains = utils::parse_csv_to_vec(db.get(constants::KNOWN_DOMAINS_KEY)?);
    log::debug!("Previously known domains: {:?}", known_domains);

    let all_domains = certstream::dump(&db)?;
    log::debug!("All domains: {:?}", all_domains);

    let new_domains = all_domains
        .iter()
        .filter(|domain| !known_domains.contains(domain.0))
        .collect::<Vec<_>>();

    log::info!("New domains: {:?}", new_domains);

    if new_domains.is_empty() {
        return Ok(None);
    }

    // Update the list of known domains in the db
    let known_domains = known_domains
        .iter()
        .chain(new_domains.iter().map(|domain| domain.0))
        .cloned()
        .collect::<Vec<_>>();
    log::debug!("New known domains: {:?}", known_domains);

    db.put(
        constants::KNOWN_DOMAINS_KEY,
        known_domains.join(",").as_bytes(),
    )?;

    Ok(Some(
        new_domains.iter().map(|domain| domain.0.clone()).collect(),
    ))
}
