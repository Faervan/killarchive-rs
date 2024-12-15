use serde::{Deserialize, Serialize};

use super::{alliance::Alliance, guild::Guild, parse_helper::QuickParse, Event, EventType};

#[derive(Hash, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub id: String,
    pub guild: Option<Guild>,
}

#[derive(Serialize)]
pub struct PlayerData {
    pub id: String,
    pub name: String,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub allies: i32,
}

impl Event {
    pub fn players(&self) -> Vec<(&Player, EventType)> {
        let mut players = vec![];
        players.push((&self.killer, EventType::Kill));
        players.push((&self.victim, EventType::Death));
        self.assists
            .iter()
            .for_each(|assist| {
                players.push((assist, EventType::Assist));
            });
        self.allies
            .iter()
            .for_each(|ally| {
                players.push((ally, EventType::Ally));
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
        })
    }
}
