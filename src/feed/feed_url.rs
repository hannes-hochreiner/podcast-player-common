#[cfg(feature = "tokio-postgres")]
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tokio-postgres")]
use std::convert::TryFrom;
#[cfg(feature = "tokio-postgres")]
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedUrl {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub url: String,
    pub status: Option<u16>,
    pub manual: bool,
    pub synced: bool,
    pub update_ts: DateTime<FixedOffset>,
}

#[cfg(feature = "tokio-postgres")]
impl TryFrom<&Row> for FeedUrl {
    type Error = anyhow::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            feed_id: row.try_get("feed_id")?,
            url: row.try_get("url")?,
            status: row.try_get("status")?,
            manual: row.try_get("manual")?,
            synced: row.try_get("synced")?,
            update_ts: row.try_get("update_ts")?,
        })
    }
}
