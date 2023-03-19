pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Common aliases
pub use std::format as f;

pub mod constants {
    // Special DB keys:
    pub const TRACKED_KEYWORDS_KEY: &str = "__tracked_keywords";

    // Path to the RocksDB database:
    pub const ROCKSDB_PATH: &str = "certstream.db";

    // Telegram chat ID to receive updates:
    pub const GROUP_CHAT_ID: &str = "-900218105";

    // Interval to send updates to the chat:
    pub const UPDATE_INTERVAL: u64 = 10; // seconds

    // CertStream URL:
    pub const CERTSTREAM_URL: &str = "wss://certstream.calidog.io/";

    // Interval to wait after a websocket connection is dropped:
    pub const WAIT_AFTER_DISCONNECT: u64 = 5; // seconds

    pub fn is_reserved_key(item: &str) -> bool {
        let special_keys = vec![TRACKED_KEYWORDS_KEY];

        special_keys.contains(&item)
    }
}
