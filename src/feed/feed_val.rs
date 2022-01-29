use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedVal {
    pub id: Uuid,
    pub title: String,
    pub synced: bool,
    pub update_ts: DateTime<FixedOffset>,
}

#[cfg(feature = "db")]
mod db {
    use anyhow::Result;
    use std::convert::TryFrom;
    use tokio_postgres::Row;

    impl TryFrom<&Row> for super::FeedVal {
        type Error = anyhow::Error;

        fn try_from(row: &Row) -> Result<Self, Self::Error> {
            Ok(Self {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                synced: row.try_get("synced")?,
                update_ts: row.try_get("update_ts")?,
            })
        }
    }

    impl crate::DbInfo for super::FeedVal {
        fn table_name() -> &'static str {
            "feed_val"
        }
    }
}
