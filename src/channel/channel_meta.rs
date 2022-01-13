#[cfg(feature = "tokio-postgres")]
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tokio-postgres")]
use std::convert::TryFrom;
#[cfg(feature = "tokio-postgres")]
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelMeta {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub active: bool,
    pub synced: bool,
    pub volume: f64,
    pub playback_rate: f64,
    pub update_ts: DateTime<FixedOffset>,
}

#[cfg(feature = "tokio-postgres")]
impl TryFrom<&Row> for ChannelMeta {
    type Error = anyhow::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            channel_id: row.try_get("channel_id")?,
            active: row.try_get("active")?,
            synced: row.try_get("synced")?,
            volume: row.try_get("volume")?,
            playback_rate: row.try_get("playback_rate")?,
            update_ts: row.try_get("update_ts")?,
        })
    }
}
