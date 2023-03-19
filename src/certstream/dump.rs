use rocksdb::{IteratorMode, Options, DB};
use std::collections::HashMap;

pub fn dump(rocksdb_path: &str) -> HashMap<String, String> {
    let mut options = Options::default();
    options.set_error_if_exists(false);
    options.create_if_missing(false);

    let db = DB::open(&options, rocksdb_path.to_owned()).unwrap();

    let iter = db.iterator(IteratorMode::Start); // Always iterates forward

    let mut domains_list = HashMap::new();
    for (key, value) in iter {
        let domain = String::from_utf8(key.to_vec()).unwrap();
        let timestamp = String::from_utf8(value.to_vec()).unwrap();

        domains_list.insert(domain, timestamp);
    }

    domains_list
}
