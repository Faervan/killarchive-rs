use alliances::handle_alliances;
use guilds::handle_guilds;
use players::handle_players;
use tokio_postgres::Client;

use crate::{albion_api::{types::Event, EVENT_URL}, error::Error};

mod alliances;
mod guilds;
mod players;

pub async fn fetch_events(client: &Client) -> Result<(), Error> {
    println!("\nFetching events...\n");

    let events = reqwest::get(format!("{EVENT_URL}?limit=50"))
        .await?
        .json::<Vec<Event>>()
        .await?;

    handle_alliances(client, &events).await?;
    handle_guilds(client, &events).await?;
    handle_players(client, &events).await?;

    for event in &events {
        println!("{} killed {}", event.killer.name, event.victim.name);
    }

    Ok(())
}
