use std::{io, sync::Arc};

use actix_web::{get, http::header::ContentType, web::{Data, Path, Query}, HttpResponse};
use tera::{Context, Tera};
use tokio_postgres::Client;

use crate::{albion_api::types::player::PlayerData, error::Error};
use super::Params;

#[get("/players")]
async fn players_index(client: Data<Arc<Client>>, tera: Data<Tera>, params: Query<Params>) -> Result<HttpResponse, Error> {
    let query = format!("
        SELECT
            players.name,
            guilds.name,
            players.kills,
            players.deaths,
            players.assists,
            players.allies
        FROM players
        JOIN guilds ON players.guild = guilds.id
        ORDER BY {} DESC, {} DESC
        LIMIT 20 OFFSET {}
    ", params.get_order(),
        params.get_secundary_order(),
        params.offset.unwrap_or(0)
    );
    let rows: Vec<PlayerData> = client
        .query(&query, &[])
        .await?
        .into_iter()
        .map(|row| PlayerData {
            name: row.get::<usize, &str>(0).to_string(),
            guild: row.get::<usize, &str>(1).to_string(),
            kills: row.get(2),
            deaths: row.get(3),
            assists: row.get(4),
            allies: row.get(5),
        })
        .collect();

    let count: Vec<i64> = client
        .query("SELECT COUNT(*) FROM players", &[])
        .await?
        .into_iter()
        .map(|row| row.get(0))
        .collect();

    let mut context = Context::new();
    context.insert("players", &rows);
    context.insert("player_count", &count[0]);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera.render(
            match params.offset.is_some() || params.order_by.is_some() {
                true => "players/rows.html",
                false => "players/index.html"
            },
            &context
        )?))
}

#[get("/players/{name}")]
async fn player_show(client: Data<Arc<Client>>, tera: Data<Tera>, path: Path<String>) -> Result<HttpResponse, Error> {
    let query = format!("
        SELECT
            players.name,
            guilds.name,
            players.kills,
            players.deaths,
            players.assists,
            players.allies
        FROM players
        JOIN guilds ON players.guild = guilds.id
        WHERE players.name = '{path}'
    ");
    let mut player = client
        .query(&query, &[])
        .await?
        .into_iter()
        .map(|row| PlayerData {
            name: row.get::<usize, &str>(0).to_string(),
            guild: row.get::<usize, &str>(1).to_string(),
            kills: row.get(2),
            deaths: row.get(3),
            assists: row.get(4),
            allies: row.get(5),
        })
    .next()
    .ok_or(io::Error::new(io::ErrorKind::NotFound, "Oh no!"))?;

    if player.guild.is_empty() {
        player.guild = "[None]".to_string();
    }

    let mut context = Context::new();
    context.insert("player", &player);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(tera.render("players/show.html", &context)?))
}
