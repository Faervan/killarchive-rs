use std::collections::HashMap;

use tokio_postgres::{types::ToSql, Client};

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
            allies,
            winrate,
            kill_fame,
            death_fame,
            fame_ratio
        ) VALUES ($1, $11, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .await?;
    let update = client
        .prepare("UPDATE guilds
            SET
                alliance = $2,
                kills = kills + $3,
                deaths = deaths + $4,
                assists = assists + $5,
                allies = allies + $6,
                winrate = $7,
                kill_fame = kill_fame + $8,
                death_fame = death_fame + $9,
                fame_ratio = $10
            WHERE id = $1
        ")
        .await?;

    let guilds = events
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<&String, (&Guild, EventCount)>, event| {
            event.guilds()
                .into_iter()
                .for_each(|(guild, ty)|
                    if let Some(previous) = acc.insert(&guild.id, (guild, ty.into())) {
                        acc.get_mut(&guild.id)
                            .map(|(_, new)| *new += previous.1);
                    }
                );
            acc
        });

    for (guild, events) in guilds.values() {
        let data: &[&(dyn ToSql + Sync)] = &[
            &guild.id,
            &guild.alliance.as_ref().map(|a| &a.id),
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
        named_data.push(&guild.name);
        if let Err(_) = client.execute(&insert, &named_data).await {
            client.execute(&update, data).await?;
        }
    }

    Ok(())
}
