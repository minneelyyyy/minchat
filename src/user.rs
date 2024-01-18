use crate::*;

use rocket::serde::Serialize;
use rusqlite::params;

#[derive(Serialize)]
pub struct User {
    id: UserID,
    username: String,
    display: Option<String>,
    avatar: String,
}

impl User {
    pub async fn get(uid: UserID) -> rocket_anyhow::Result<User> {
        let conn = open_db().await?;
    
        let user = conn.call(move |conn| {
            let user = conn.query_row("SELECT id, username, display, avatar FROM users WHERE id=?1", params![uid], |row| {
                Ok(User {
                    id: uid,
                    username: row.get(1)?,
                    display: row.get(2)?,
                    avatar: row.get(3)?
                })
            })?;
    
            Ok(user)
        }).await?;
    
        Ok(user)
    }
}

