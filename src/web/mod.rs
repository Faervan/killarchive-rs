use std::sync::Arc;

use rocket::{get, response::content::RawHtml, routes, State};
use tokio_postgres::Client;

pub async fn rocket(client: Arc<Client>) -> Result<rocket::Rocket<rocket::Ignite>, rocket::Error> {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/players", routes![players])
        .manage(client)
        .launch()
        .await
}

#[get("/")]
fn hello() -> RawHtml<String> {
    RawHtml(format!("<h1>killarchive-rs</h1><ul>{}</ul>",
        ["Players"]
        .into_iter()
        .map(|item|
            format!("<li><a href=\"{}\">{item}</a></li>",
                item.to_lowercase()
            )
        ).collect::<Vec<String>>()
        .concat()
    ))
}

#[get("/")]
async fn players(state: &State<Arc<Client>>) -> RawHtml<String> {
    let rows = state
        .query("SELECT * FROM players ORDER BY kills DESC LIMIT 20", &[])
        .await
        .expect("query failed");
    let rows: Vec<String> = rows.into_iter()
        .map(|row| {
            format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                row.get::<usize, &str>(1),
                row.get::<usize, &str>(2),
                row.get::<usize, i32>(3),
                row.get::<usize, i32>(4)
            )
        })
        .collect();
    RawHtml(format!("<html><body><table><tr><th>Player</th><th>Guild</th><th>Kills</th><th>Deaths</th></tr>{}</table></body></html>", rows.concat()))
}
