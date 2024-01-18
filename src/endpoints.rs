use crate::*;
extern crate tokio_rusqlite as sql;

use rocket::serde::{Deserialize, json::Json};

use message::Message;
use user::User;

pub fn routes() -> Vec<rocket::Route> {
    routes![index, send_message, get_user]
}

#[get("/")]
async fn index() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open("pages/v1docs.html").await.ok()
}

#[derive(Deserialize)]
struct SendMessageBody {
    content: String
}

#[post("/channels/<channel>/messages", data = "<message>")]
async fn send_message(channel: ChannelID, message: Json<SendMessageBody>) -> rocket_anyhow::Result<Json<Message>> {
    let msg = Message::new(message.content.clone().trim().to_string(), channel, 0).await?;
    
    // TODO: First, send the message to users
    
    // Second, push the message to the database
    let conn = open_db().await?;
    msg.write_to_db(&conn).await?;

    Ok(Json(msg))
}

#[get("/users/<id>")]
async fn get_user(id: UserID) -> rocket_anyhow::Result<Json<User>> {
    User::get(id).await.map(Json::from)
}