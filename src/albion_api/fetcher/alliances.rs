use std::collections::HashMap;

use tokio_postgres::{types::ToSql, Client};

use crate::{albion_api::types::{alliance::Alliance, Event, EventCount}, error::Error};

pub async fn handle_alliances(client: &Client, events: &Vec<Event>) -> Result<(), Error> {
    let insert = client
        .prepare("INSERT INTO alliances (
            id,
            name,
            kills,
            deaths,
            assists,
            allies,
            winrate,
            kill_fame,
            death_fame,
            fame_ratio
        ) VALUES ($1, $10, $2, $3, $4, $5, $6, $7, $8, $9)")
        .await?;
    let update = client
        .prepare("UPDATE alliances
            SET
                kills = kills + $2,
                deaths = deaths + $3,
                assists = assists + $4,
                allies = allies + $5,
                winrate = $6,
                kill_fame = kill_fame + $7,
                death_fame = death_fame + $8,
                fame_ratio = $9
            WHERE id = $1
        ")
        .await?;

    let alliances = events
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<&String, (&Alliance, EventCount)>, event| {
            event.alliances()
                .into_iter()
                .for_each(|(alliance, ty)|
                    if let Some(previous) = acc.insert(&alliance.id, (alliance, ty.into())) {
                        acc.get_mut(&alliance.id)
                            .map(|(_, new)| *new += previous.1);
                    }
                );
            acc
        });

    for (alliance, events) in alliances.values() {
        let data: &[&(dyn ToSql + Sync)] = &[
            &alliance.id,
            &events.kills,
            &events.deaths,
            &events.assists,
            &events.allies,
            &(((events.kills as f32 / (events.kills + events.deaths) as f32) * 100.).round() as i16),
            &events.kill_fame,
            &events.death_fame,
            &(((events.kill_fame as f32 / (events.kill_fame + events.death_fame) as f32) * 100.).round() as i16),
        ];
        let mut named_data = data.to_vec().clone();
        named_data.push(&alliance.name);
        if let Err(_) = client.execute(&insert, &named_data).await {
            client.execute(&update, data).await?;
        }
    }

    Ok(())
}
