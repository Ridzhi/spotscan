// дата
// площадка
// непрерывные слоты

use std::collections::HashMap;
use time::{Duration, Time};
use crate::spot::Slot;

pub type TgUserId = i64;
pub type TgAccessHash = i64;

pub struct User {
    pub id: i64,
    pub tg_user_id: TgUserId,
    pub tg_user_access_hash: TgAccessHash,
}

pub struct Settings {
    pub is_active: bool,
    pub defaults: WindowDefaults,
    pub slots: HashMap<time::Weekday, WindowSettings>,
}

pub struct WindowDefaults {
    pub duration: Duration,
    pub starts: Option<Time>,
    pub ends: Option<Time>,
}

pub struct WindowSettings {
    pub duration: Duration,
    pub starts: Time,
    pub ends: Time,
}
// слот должен проходить проверку на:
//  - начинается не раньше указанной границы
//  - заканчивается не позже указанной границы
//  - имеет длительность не меньше заданной

// pub struct Slots()

pub type Schedule = HashMap<u8, Vec<TimeWindow>>;
// pub struct Schedule(HashMap<String, Vec<TimeWindow>>);
// pub struct Day(PrimitiveDateTime, Vec<Playground>);
// pub struct Playground {
//     pub number: u8,
//     pub time_windows: Vec<TimeWindow>
// }



#[derive(Clone, Debug)]
pub struct TimeWindow {
    pub start: Time,
    pub end: Time,
}