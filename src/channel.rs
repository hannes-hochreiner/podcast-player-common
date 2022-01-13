pub mod channel_meta;
pub mod channel_val;

use channel_meta::ChannelMeta;
use channel_val::ChannelVal;
use chrono::{DateTime, FixedOffset, Utc};
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
                id: uuid::Uuid::new_v4(),
                channel_id: val.id,
                active: false,
                synced: false,
                playback_rate: 1.5,
                volume: 0.5,
                update_ts: Utc::now().into(),
            },
        }
    }
}
