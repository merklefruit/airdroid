pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Common aliases
pub use std::format as f;

pub mod constants {
    // Special DB keys
    pub const TRACKED_KEYWORDS_KEY: &str = "__tracked_keywords";
    pub const KNOWN_DOMAINS_KEY: &str = "__known_domains";

    /// Path to the RocksDB database
    pub const ROCKSDB_PATH: &str = "certstream.db";

    /// Telegram chat ID or channel name to receive updates on
    pub const TG_CHANNEL_ID: &str = "@airdroidsbotchannel";

    /// Interval to send updates to the chat (seconds)
    pub const UPDATE_INTERVAL: u64 = 30;

    /// CertStream URL
    pub const CERTSTREAM_URL: &str = "wss://certstream.calidog.io/";

    /// Interval to wait after a websocket connection is dropped (seconds)
    pub const WAIT_AFTER_DISCONNECT: u64 = 4;

    pub fn is_reserved_key(item: &str) -> bool {
        let special_keys = vec![TRACKED_KEYWORDS_KEY, KNOWN_DOMAINS_KEY];

        special_keys.contains(&item)
    }
}
