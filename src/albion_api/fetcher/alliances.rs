use std::collections::HashMap;

use tokio_postgres::Client;

use crate::{albion_api::types::{alliance::Alliance, Event, EventCount}, error::Error};

pub async fn handle_alliances(client: &Client, events: &Vec<Event>) -> Result<(), Error> {
    let insert = client
        .prepare("INSERT INTO alliances (
            id,
            name,
            kills,
            deaths,
            assists,
            allies
        ) VALUES ($1, $2, $3, $4, $5, $6)")
        .await?;
    let update = client
        .prepare("UPDATE alliances
            SET
                kills = kills + $2,
                deaths = deaths + $3,
                assists = assists + $4,
                allies = allies + $5
            WHERE id = $1
        ")
        .await?;

    let alliances = events
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<&Alliance, EventCount>, event| {
        event.alliances()
            .into_iter()
            .for_each(|(alliance, ty)|
                if let Some(previous) = acc.insert(alliance, ty.into()) {
                    acc.get_mut(alliance)
                        .map(|new| *new += previous);
                }
            );
        acc
    });

    for (alliance, events) in &alliances {
        if let Err(_) = client.execute(&insert, &[
            &alliance.id,
            &alliance.name,
            &events.kills,
            &events.deaths,
            &events.assists,
            &events.allies,
        ]).await {
            client.execute(&update, &[
                &alliance.id,
                &events.kills,
                &events.deaths,
                &events.assists,
                &events.allies
            ]).await?;
        }
    }

    Ok(())
}
