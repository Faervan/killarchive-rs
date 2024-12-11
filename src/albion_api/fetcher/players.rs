use std::collections::HashMap;

use tokio_postgres::Client;

use crate::{albion_api::types::{player::Player, Event, EventCount}, error::Error};

pub async fn handle_players(client: &Client, events: &Vec<Event>) -> Result<(), Error> {
    let insert = client
        .prepare("INSERT INTO players (
            id,
            name,
            guild,
            kills,
            deaths,
            assists,
            allies
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .await?;
    let update = client
        .prepare("UPDATE players
            SET
                kills = kills + $2,
                deaths = deaths + $3,
                assists = assists + $4,
                allies = allies + $5
            WHERE id = $1
        ")
        .await?;

    let players = events
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<&Player, EventCount>, event| {
        event.players()
            .into_iter()
            .for_each(|(player, ty)|
                if let Some(previous) = acc.insert(player, ty.into()) {
                    acc.get_mut(player)
                        .map(|new| *new += previous);
                }
            );
        acc
    });

    for (player, events) in &players {
        if let Err(_) = client.execute(&insert, &[
            &player.id,
            &player.name,
            &player.guild.as_ref().map(|g| &g.id),
            &events.kills,
            &events.deaths,
            &events.assists,
            &events.allies,
        ]).await {
            client.execute(&update, &[
                &player.id,
                &events.kills,
                &events.deaths,
                &events.assists,
                &events.allies
            ]).await?;
        }
    }

    Ok(())
}
