// Ported from: https://github.com/hrbrmstr/certstream-rust/tree/batman
// Modified to filter specific domains for a telegram bot.

use super::json_types;
use crate::{prelude::*, utils};
use chrono::prelude::*;
use futures_util::StreamExt;
use itertools::Itertools;
use regex::Regex;
use rocksdb::{Options, DB};
use std::{
    process,
    sync::{Arc, Mutex},
    thread, time,
};
use tokio_tungstenite::connect_async;

// in the deserialization part, the type of the returnd, parsed JSON gets wonky
macro_rules! assert_types {
  ($($var:ident : $ty:ty),*) => { $(let _: & $ty = & $var;)* }
}

pub async fn scrape(db: Arc<DB>) -> Result<()> {
    ctrlc::set_handler(move || {
        process::exit(0x0000);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        // server is likely to drop connections
        let certstream_url = url::Url::parse(constants::CERTSTREAM_URL).unwrap(); // we need an actual Url type

        let mut keywords_to_track =
            utils::parse_csv_to_vec(db.get(constants::TRACKED_KEYWORDS_KEY)?);

        let re = Regex::new(&keywords_to_track.join("|")).unwrap();

        // connect to CertStream's encrypted websocket interface
        let (wss_stream, _response) = connect_async(certstream_url)
            .await
            .expect("Failed to connect");

        // the WebSocketStrem has sink/stream (read/srite) components; this is how we get to them
        let (mut _write, read) = wss_stream.split();

        // process messages as they come in
        let read_future = read.for_each(|message| async {
            match message {
                Ok(msg) => {
                    // we have the websockets message bytes as a str

                    if let Ok(json_data) = msg.to_text() {
                        // did the bytes convert to text ok?
                        if !json_data.is_empty() {
                            // do we actually have semi-valid JSON?
                            match serde_json::from_str(json_data) {
                                // if deserialization works
                                Ok(record) => {
                                    // then derserialize JSON

                                    assert_types! { record: json_types::CertStream }

                                    for dom in
                                        record.data.leaf_cert.all_domains.into_iter().unique()
                                    {
                                        // Only add domains that match a specific pattern
                                        let lowercase_dom = dom.to_ascii_lowercase();

                                        if re.is_match(&lowercase_dom) {
                                            log::info!("Found new domain: {}", lowercase_dom);
                                            // Add timestamp as "last-seen-at" value to current timestamp
                                            db.put(lowercase_dom, Utc::now().to_string()).unwrap();
                                        }
                                    }
                                }

                                Err(err) => {
                                    eprintln!("{}", err)
                                }
                            }
                        }
                    }
                }

                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        });

        read_future.await;

        eprintln!(
            "Server disconnected… waiting {} seconds and retrying…",
            constants::WAIT_AFTER_DISCONNECT
        );

        // wait for a bit to be kind to the server
        thread::sleep(time::Duration::from_secs(constants::WAIT_AFTER_DISCONNECT));
    }

    Ok(())
}
