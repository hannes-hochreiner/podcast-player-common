use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemVal {
    pub id: Uuid,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub enclosure_type: String,
    pub enclosure_url: String,
    pub channel_id: Uuid,
    pub size: i64,
    pub update_ts: DateTime<FixedOffset>,
}

impl ItemVal {
    pub fn needs_update(&self, enclosure_type: &String, enclosure_url: &String) -> bool {
        !(&self.enclosure_type == enclosure_type && &self.enclosure_url == enclosure_url)
    }
}

#[cfg(feature = "db")]
mod db {
    use anyhow::Result;
    use std::convert::TryFrom;
    use tokio_postgres::Row;

    impl TryFrom<&Row> for super::ItemVal {
        type Error = anyhow::Error;

        fn try_from(row: &Row) -> Result<Self, Self::Error> {
            Ok(Self {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                date: row.try_get("date")?,
                enclosure_type: row.try_get("enclosure_type")?,
                enclosure_url: row.try_get("enclosure_url")?,
                channel_id: row.try_get("channel_id")?,
                size: row.try_get("size")?,
                update_ts: row.try_get("update_ts")?,
            })
        }
    }

    impl crate::DbInfo for super::ItemVal {
        fn table_name() -> &'static str {
            "item_val"
        }
    }
}
