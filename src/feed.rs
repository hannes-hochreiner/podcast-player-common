pub mod feed_meta;
pub mod feed_val;

use chrono::{DateTime, FixedOffset};
use feed_meta::FeedMeta;
use feed_val::FeedVal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Feed {
    pub val: FeedVal,
    pub meta: FeedMeta,
}

impl Feed {
    pub fn get_val_update(&self) -> &DateTime<FixedOffset> {
        &self.val.update_ts
    }
}

impl From<&FeedVal> for Feed {
    fn from(val: &FeedVal) -> Self {
        Self {
            val: val.clone(),
            meta: FeedMeta {
                id: val.id,
                synced: false,
            },
        }
    }
}
