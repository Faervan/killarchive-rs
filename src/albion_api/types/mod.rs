use std::ops::AddAssign;

use player::Player;
use serde::Deserialize;

mod parse_helper;
pub mod alliance;
pub mod guild;
pub mod player;

#[derive(Deserialize)]
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
}

pub enum EventType {
    Kill,
    Death,
    Assist,
    Ally,
}

#[derive(Default)]
pub struct EventCount {
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub allies: i32,
}

impl From<EventType> for EventCount {
    fn from(value: EventType) -> Self {
        match value {
            EventType::Kill => EventCount {
                kills: 1,
                ..Default::default()
            },
            EventType::Death => EventCount {
                deaths: 1,
                ..Default::default()
            },
            EventType::Assist => EventCount {
                assists: 1,
                ..Default::default()
            },
            EventType::Ally => EventCount {
                allies: 1,
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
    }
}
