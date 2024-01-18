use minchat::*;

fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/api/v1", endpoints::routes())
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let conn = open_db().await?;

    let _ = conn.call(|conn| {
        conn.execute(
            "CREATE TABLE messages (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                channel INTEGER,
                author INTEGER
            )", [])?;

        conn.execute(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                display TEXT,
                avatar TEXT
            )", [])?;

        Ok(())
    }).await;

    rocket().launch().await?;
    Ok(())
}