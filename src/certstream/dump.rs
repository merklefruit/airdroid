use crate::prelude::*;
use rocksdb::{IteratorMode, DB};
use std::{collections::HashMap, sync::Arc};

pub fn dump(db: &Arc<DB>) -> Result<HashMap<String, String>> {
    let iter = db.iterator(IteratorMode::Start); // Always iterates forward

    let mut domains_list = HashMap::new();

    for (key, value) in iter {
        let domain = String::from_utf8(key.to_vec()).unwrap();

        if constants::is_reserved_key(&domain) {
            continue;
        }

        log::debug!("Domain: {}", domain);

        let timestamp = String::from_utf8(value.to_vec()).unwrap();

        domains_list.insert(domain, timestamp);
    }

    Ok(domains_list)
}
