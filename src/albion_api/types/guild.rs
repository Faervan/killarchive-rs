use super::{alliance::Alliance, Event, EventType};

#[derive(Hash, PartialEq, Eq)]
pub struct Guild {
    pub name: String,
    pub id: String,
    pub alliance: Option<Alliance>,
}

impl Event {
    pub fn guilds(&self) -> Vec<(&Guild, EventType)> {
        let mut guilds = vec![];
        self.killer.guild
            .as_ref()
            .map(|g| guilds.push((g, EventType::Kill)));
        self.victim.guild
            .as_ref()
            .map(|g| guilds.push((g, EventType::Death)));
        self.assists
            .iter()
            .for_each(|assist| {
                assist.guild
                    .as_ref()
                    .map(|g| guilds.push((g, EventType::Assist)));
            });
        self.allies
            .iter()
            .for_each(|ally| {
                ally.guild
                    .as_ref()
                    .map(|g| guilds.push((g, EventType::Ally)));
            });
        guilds
    }
}
