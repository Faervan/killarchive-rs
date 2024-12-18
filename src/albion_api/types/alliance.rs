use super::{player::Player, Event, EventFame};

#[derive(Hash, PartialEq, Eq, Debug)]
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
        for (player, fame) in self.players() {
            player.alliance().map(|a|
                alliances.push((a, fame))
            );
        }
        alliances
    }
}
