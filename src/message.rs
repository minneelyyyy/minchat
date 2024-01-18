use crate::*;
extern crate tokio_rusqlite as sql;

use rocket::serde::Serialize;
use rusqlite::params;

#[derive(Clone, Serialize)]
pub(crate) struct Message {
    id: MessageID,
    content: String,
    channel: ChannelID,
    author: UserID,
}

async fn message_exists(id: Snowflake, conn: &sql::Connection) -> anyhow::Result<bool> {
    let found: bool = conn.call(move |conn| {
        let found = conn.query_row("SELECT id FROM messages WHERE id=?1",
            params![id], |row| row.get::<_, Snowflake>(0)).is_ok();
        Ok(found)
    }).await?;

    Ok(found)
}

async fn new_message_id(conn: &sql::Connection) -> anyhow::Result<Snowflake> {
    let mut n: Snowflake = rand::random();

    while message_exists(n, conn).await? {
        n = rand::random();
    }

    Ok(n)
}

impl Message {
    pub async fn new(content: String, channel: ChannelID, author: UserID) -> anyhow::Result<Self> {
        let conn = open_db().await?;

        Ok(Self {
            id: new_message_id(&conn).await?,
            content, channel, author
        })
    }

    pub async fn write_to_db(&self, conn: &sql::Connection) -> anyhow::Result<()> {
        let clone = self.clone();

        conn.call(move |conn| {
            conn.execute("INSERT INTO messages (id, content, channel, author) VALUES (?1, ?2, ?3, ?4)",
                params![clone.id, clone.content, clone.channel, clone.author]
            )?;

            Ok(())
        }).await?;

        Ok(())
    }
}