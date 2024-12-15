use std::sync::Arc;

use actix_web::{get, http::header::ContentType, web::{Data, Query}, HttpResponse};
use tera::{Context, Tera};
use tokio_postgres::Client;

use crate::albion_api::types::player::PlayerData;
use super::Params;

#[get("/players")]
async fn players_index(state: Data<Arc<Client>>, tera: Data<Tera>, params: Query<Params>) -> HttpResponse {
    let query = "SELECT * FROM players ORDER BY kills DESC, assists DESC LIMIT 20 OFFSET";
    let rows: Vec<PlayerData> = state
        .query(&format!("{query} {}", params.offset.unwrap_or(0)), &[])
        .await
        .expect("query failed")
        .into_iter()
        .map(|row| PlayerData {
                id: row.get::<usize, &str>(1).to_string(),
                name: row.get::<usize, &str>(2).to_string(),
                kills: row.get(3),
                deaths: row.get(4),
                assists: row.get(5),
                allies: row.get(6),
            }
        )
        .collect();

    let mut context = Context::new();
    context.insert("players", &rows);

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera.render(match params.offset {
            Some(_) => "players/rows.html",
            None => "players/index.html"
        }, &context).unwrap())
}
