use std::ops::AddAssign;

use player::Player;
use serde::Deserialize;

mod parse_helper;
pub mod alliance;
pub mod guild;
pub mod player;

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "EventId")]
    pub id: usize,
    #[serde(rename = "Killer")]
    pub killer: Player,
    #[serde(rename = "Victim")]
    pub victim: Player,
    #[serde(rename = "Participants")]
    pub assists: Vec<Player>,
    #[serde(rename = "GroupMembers")]
    pub allies: Vec<Player>,
    #[serde(rename = "TotalVictimKillFame")]
    pub total_fame: i64,
}

pub enum EventType {
    Kill,
    Death,
    Assist,
    Ally,
}

pub struct EventFame {
    value: i64,
    ty: EventType,
}

#[derive(Default)]
pub struct EventCount {
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub allies: i32,
    pub kill_fame: i64,
    pub death_fame: i64,
}

impl From<EventFame> for EventCount {
    fn from(fame: EventFame) -> Self {
        match fame.ty {
            EventType::Kill => EventCount {
                kills: 1,
                kill_fame: fame.value,
                ..Default::default()
            },
            EventType::Death => EventCount {
                deaths: 1,
                death_fame: fame.value,
                ..Default::default()
            },
            EventType::Assist => EventCount {
                assists: 1,
                ..Default::default()
            },
            EventType::Ally => EventCount {
                allies: 1,
                kill_fame: fame.value,
                ..Default::default()
            },
        }
    }
}

impl AddAssign for EventCount {
    fn add_assign(&mut self, rhs: Self) {
            self.kills += rhs.kills;
            self.deaths += rhs.deaths;
            self.assists += rhs.assists;
            self.allies += rhs.allies;
            self.kill_fame += rhs.kill_fame;
            self.death_fame += rhs.death_fame;
    }
}
