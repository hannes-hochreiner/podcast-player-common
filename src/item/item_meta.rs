#[cfg(feature = "tokio-postgres")]
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tokio-postgres")]
use std::convert::TryFrom;
#[cfg(feature = "tokio-postgres")]
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemMeta {
    pub id: Uuid,
    pub item_id: Uuid,
    pub new: bool,
    pub download: bool,
    pub download_status: DownloadStatus,
    pub current_time: Option<f64>,
    pub play_count: u32,
    pub synced: bool,
    pub update_ts: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DownloadStatus {
    NotRequested,
    Pending,
    InProgress,
    Ok,
    Error,
}

#[cfg(feature = "tokio-postgres")]
impl TryFrom<&Row> for ItemMeta {
    type Error = anyhow::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            item_id: row.try_get("item_id")?,
            new: row.try_get("new")?,
            download: row.try_get("download")?,
            download_status: row.try_get("download_status")?,
            current_time: row.try_get("current_time")?,
            play_count: row.try_get("play_count")?,
            synced: row.try_get("synced")?,
            update_ts: row.try_get("update_ts")?,
        })
    }
}
