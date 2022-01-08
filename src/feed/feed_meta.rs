use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FeedMeta {
    pub id: Uuid,
    pub synced: bool,
}
