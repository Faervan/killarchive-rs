use std::collections::HashMap;

use tokio_postgres::{types::ToSql, Client};

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
            allies,
            winrate,
            kill_fame,
            death_fame,
            fame_ratio
        ) VALUES ($1, $11, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .await?;
    let update = client
        .prepare("UPDATE players
            SET
                guild = $2,
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
        let data: &[&(dyn ToSql + Sync)] = &[
            &player.id,
            &player.guild.as_ref().map(|g| &g.id),
            &events.kills,
            &events.deaths,
            &events.assists,
            &events.allies,
            &(((events.kills as f32 / (events.kills + events.deaths) as f32) * 100.).round() as i16),
            &events.kill_fame,
            &events.death_fame,
            &(((events.kill_fame as f32 / (events.kill_fame + events.death_fame) as f32) * 100.).round() as i16),
            &player.name,
        ];
        if let Err(_) = client.execute(&insert, data).await {
            // idk why this shit works for alliances and guilds and keeps failing for players...?
            let _ = client.execute(&update, data).await;
        }
    }

    Ok(())
}
