use std::{sync::Arc, time::Duration};

use alliances::handle_alliances;
use guilds::handle_guilds;
use players::handle_players;
use tokio::time::sleep;
use tokio_postgres::Client;

use crate::{albion_api::{types::Event, EVENT_URL}, error::Error};

mod alliances;
mod guilds;
mod players;

pub async fn schedule(client: Arc<Client>) -> Result<(), Error> {
    println!("\nStarting to schedule event fetching...\n");
    loop {
        tokio::spawn(fetch_events(client.clone()));
        sleep(Duration::from_secs(30)).await;
    }
}

pub async fn fetch_events(client: Arc<Client>) -> Result<(), Error> {
    println!("\nFetching events...\n");

    let events = reqwest::get(format!("{EVENT_URL}?limit=50"))
        .await?
        .json::<Vec<Event>>()
        .await?;

    match validate_events(&client, &events).await {
        Ok(()) => {
            handle_alliances(&client, &events).await?;
            handle_guilds(&client, &events).await?;
            handle_players(&client, &events).await?;

            for event in &events {
                println!("{} killed {}", event.killer.name, event.victim.name);
            }
        }
        Err(err) => println!("The fetched events are already tracked in the database! err: {err:?}")
    }

    Ok(())
}

async fn validate_events(client: &Client, events: &Vec<Event>) -> Result<(), Error> {
    client.batch_execute("
        DELETE FROM cached_events WHERE timestamp < NOW() - INTERVAL '1 hour'
    ").await?;
    let insert = client.prepare("
        INSERT INTO cached_events (id) VALUES ($1)
    ").await?;

    for event in events {
        client.execute(&insert, &[&(event.id as i32)]).await?;
    }
    Ok(())
}
