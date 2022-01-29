#[cfg(feature = "db")]
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[cfg(feature = "db")]
use std::convert::TryFrom;
#[cfg(feature = "db")]
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedUrl {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub url: String,
    pub status: Option<i16>,
    pub manual: bool,
    pub synced: bool,
    pub update_ts: DateTime<FixedOffset>,
}

impl Ord for FeedUrl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.status, &other.status) {
            (Some(status), None) => {
                if status >= 200 && status < 300 {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
            (Some(status_self), Some(status_other)) => {
                if !(status_self >= 200 && status_self < 300)
                    && (*status_other >= 200 && *status_other < 300)
                {
                    return Ordering::Less;
                } else if (status_self >= 200 && status_self < 300)
                    && !(*status_other >= 200 && *status_other < 300)
                {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            }
            (None, Some(status)) => {
                if *status >= 200 && *status < 300 {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            (None, None) => return Ordering::Equal,
        }
    }
}

impl PartialOrd for FeedUrl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FeedUrl {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for FeedUrl {}

#[cfg(feature = "db")]
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
