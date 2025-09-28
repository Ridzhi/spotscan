// дата
// площадка
// непрерывные слоты

use std::collections::HashMap;
use time::Time;

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