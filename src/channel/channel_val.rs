use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelVal {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub feed_id: Uuid,
    pub update_ts: DateTime<FixedOffset>,
}

impl ChannelVal {
    pub fn needs_update(&self, description: &String, image: &Option<String>) -> bool {
        !(&self.description == description && &self.image == image)
    }
}

#[cfg(feature = "db")]
mod db {
    use anyhow::Result;
    use std::convert::TryFrom;
    use tokio_postgres::Row;

    impl TryFrom<&Row> for super::ChannelVal {
        type Error = anyhow::Error;

        fn try_from(row: &Row) -> Result<Self, Self::Error> {
            Ok(Self {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                description: row.try_get("description")?,
                image: match row.try_get("image") {
                    Ok(i) => Some(i),
                    Err(_) => None,
                },
                feed_id: row.try_get("feed_id")?,
                update_ts: row.try_get("update_ts")?,
            })
        }
    }

    impl crate::DbInfo for super::ChannelVal {
        fn table_name() -> &'static str {
            "channel_val"
        }
    }
}
