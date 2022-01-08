pub mod channel_meta;
pub mod channel_val;

use channel_meta::ChannelMeta;
use channel_val::ChannelVal;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Channel {
    pub val: ChannelVal,
    pub meta: ChannelMeta,
}

impl Channel {
    pub fn get_val_update(&self) -> &DateTime<FixedOffset> {
        &self.val.update_ts
    }
}

impl From<&ChannelVal> for Channel {
    fn from(val: &ChannelVal) -> Self {
        Self {
            val: val.clone(),
            meta: ChannelMeta {
                id: val.id,
                active: false,
                synced: false,
            },
        }
    }
}
