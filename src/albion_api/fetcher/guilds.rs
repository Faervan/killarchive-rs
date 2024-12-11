use std::collections::HashMap;

use tokio_postgres::Client;

use crate::{albion_api::types::{guild::Guild, Event, EventCount}, error::Error};

pub async fn handle_guilds(client: &Client, events: &Vec<Event>) -> Result<(), Error> {
    let insert = client
        .prepare("INSERT INTO guilds (
            id,
            name,
            alliance,
            kills,
            deaths,
            assists,
            allies
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .await?;
    let update = client
        .prepare("UPDATE guilds
            SET
                kills = kills + $2,
                deaths = deaths + $3,
                assists = assists + $4,
                allies = allies + $5
            WHERE id = $1
        ")
        .await?;

    let guilds = events
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<&Guild, EventCount>, event| {
        event.guilds()
            .into_iter()
            .for_each(|(guild, ty)|
                if let Some(previous) = acc.insert(guild, ty.into()) {
                    acc.get_mut(guild)
                        .map(|new| *new += previous);
                }
            );
        acc
    });

    for (guild, events) in &guilds {
        if let Err(_) = client.execute(&insert, &[
            &guild.id,
            &guild.name,
            &guild.alliance.as_ref().map(|a| &a.id),
            &events.kills,
            &events.deaths,
            &events.assists,
            &events.allies,
        ]).await {
            client.execute(&update, &[
                &guild.id,
                &events.kills,
                &events.deaths,
                &events.assists,
                &events.allies
            ]).await?;
        }
    }

    Ok(())
}
