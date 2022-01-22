mod channel;
mod feed;
mod item;

pub use channel::*;
pub use feed::*;
pub use item::*;

#[cfg(feature = "db")]
pub trait DbInfo {
    fn table_name() -> &'static str;
}
