
#[macro_use] pub extern crate rocket;

pub type Snowflake = u64;
pub type MessageID = Snowflake;
pub type ChannelID = Snowflake;
pub type UserID = Snowflake;

pub mod message;
pub mod endpoints;
pub mod user;

pub async fn open_db() -> tokio_rusqlite::Result<tokio_rusqlite::Connection> {
    tokio_rusqlite::Connection::open("secrets/data.db").await
}