use serde::{Deserialize, Serialize};

use super::{alliance::Alliance, guild::Guild, parse_helper::QuickParse, Event, EventFame, EventType};

#[derive(Hash, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub id: String,
    pub guild: Option<Guild>,
    pub kill_fame: i64,
}

#[derive(Serialize)]
pub struct PlayerData {
    pub name: String,
    pub guild: String,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub allies: i32,
}

impl Event {
    pub fn players(&self) -> Vec<(&Player, EventFame)> {
        let mut players = vec![];
        players.push((&self.killer, EventFame {
            value: self.killer.kill_fame,
            ty: EventType::Kill,
        }));
        players.push((&self.victim, EventFame {
            value: self.total_fame,
            ty: EventType::Death,
        }));
        self.assists
            .iter()
            .for_each(|assist| {
                players.push((assist, EventFame {
                    value: 0,
                    ty: EventType::Assist,
                }));
            });
        self.allies
            .iter()
            .for_each(|ally| {
                players.push((ally, EventFame {
                    value: ally.kill_fame,
                    ty: EventType::Ally,
                }));
            });
        players
    }
}

impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

        let mut map: serde_json::Map<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;

        let alliance = map.get_key::<D>("AllianceId")
            .map(|id| Alliance {
                name: map.key("AllianceName"),
                id,
            });

        let guild = map.get_key::<D>("GuildId")
            .map(|id| Guild {
                name: map.key("GuildName"),
                id,
                alliance: alliance.ok(),
            });

        Ok(Player {
            name: map.key("Name"),
            id: map.key("Id"),
            guild: guild.ok(),
            kill_fame: map.key("KillFame").parse::<i64>().expect("Failed to parse KillFame as i64"),
        })
    }
}
