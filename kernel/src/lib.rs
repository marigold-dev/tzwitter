// src/lib.rs
use host::{rollup_core::RawRollupCore, runtime::Runtime};
use kernel::kernel_entry;

mod constants;
mod core;
mod message;
mod stages;
mod storage;

use message::Message;
use stages::stage_one;
use storage::store_tweet;

fn entry<Host: RawRollupCore>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");
    let messages = stage_one(host);

    // TODO: extract this into a new stage
    messages.iter().for_each(|message| {
        let Message::Tweet(tweet) = message;
        let _ = store_tweet(host, tweet);
    });
}

kernel_entry!(entry);
