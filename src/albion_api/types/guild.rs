use super::{alliance::Alliance, Event, EventFame, EventType};

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Guild {
    pub name: String,
    pub id: String,
    pub alliance: Option<Alliance>,
}

impl Event {
    pub fn guilds(&self) -> Vec<(&Guild, EventFame)> {
        let mut guilds = vec![];
        self.killer.guild
            .as_ref()
            .map(|g| guilds.push(
                (g, EventFame {
                    value: self.killer.kill_fame,
                    ty: EventType::Kill,
                })
            ));
        self.victim.guild
            .as_ref()
            .map(|g| guilds.push(
                (g, EventFame {
                    value: self.total_fame,
                    ty: EventType::Death,
                })
            ));
        self.assists
            .iter()
            .for_each(|assist| {
                assist.guild
                    .as_ref()
                    .map(|g| guilds.push(
                        (g, EventFame {
                            value: 0,
                            ty: EventType::Assist,
                        })
                    ));
            });
        self.allies
            .iter()
            .for_each(|ally| {
                ally.guild
                    .as_ref()
                    .map(|g| guilds.push(
                        (g, EventFame {
                            value: ally.kill_fame,
                            ty: EventType::Ally,
                        })
                    ));
            });
        guilds
    }
}
