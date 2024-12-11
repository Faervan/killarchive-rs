use super::{player::Player, Event, EventType};

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
    pub fn alliances(&self) -> Vec<(&Alliance, EventType)> {
        let mut alliances = vec![];
        self.killer.alliance()
            .map(|a| alliances.push((a, EventType::Kill)));
        self.victim.alliance()
            .map(|a| alliances.push((a, EventType::Death)));
        self.assists
            .iter()
            .for_each(|assist| {
                assist.alliance()
                    .map(|a| alliances.push((a, EventType::Assist)));
            });
        self.allies
            .iter()
            .for_each(|ally| {
                ally.alliance()
                    .map(|a| alliances.push((a, EventType::Ally)));
            });
        alliances
    }
}
