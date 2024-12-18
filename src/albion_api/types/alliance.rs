use super::{player::Player, Event, EventFame, EventType};

#[derive(Hash, PartialEq, Eq)]
pub struct Alliance {
    pub name: String,
    pub id: String,
}

impl Player {
    fn alliance(&self) -> Option<&Alliance> {
        self.guild
            .as_ref()
            .and_then(|guild| guild.alliance.as_ref())
    }
}

impl Event {
    pub fn alliances(&self) -> Vec<(&Alliance, EventFame)> {
        let mut alliances = vec![];
        self.killer.alliance()
            .map(|a| alliances.push(
                (a, EventFame {
                    value: self.killer.kill_fame,
                    ty: EventType::Kill,
                })
            ));
        self.victim.alliance()
            .map(|a| alliances.push(
                (a, EventFame {
                    value: self.total_fame,
                    ty: EventType::Death,
                })
            ));
        self.assists
            .iter()
            .filter(|a| a.id == self.killer.id)
            .for_each(|assist| {
                assist.alliance()
                    .map(|a| alliances.push(
                        (a, EventFame {
                            value: 0,
                            ty: EventType::Assist,
                        })
                    ));
            });
        self.allies
            .iter()
            .filter(|a| a.id == self.killer.id)
            .for_each(|ally| {
                ally.alliance()
                    .map(|a| alliances.push(
                        (a, EventFame {
                            value: ally.kill_fame,
                            ty: EventType::Ally,
                        })
                    ));
            });
        alliances
    }
}
