use serde::Deserialize;
use serde::de::{Deserializer, Error};
use time::Time;
use time::macros::format_description;
use crate::domain::{Schedule, TimeWindow};

#[derive(Deserialize)]
pub struct Response(pub Vec<Slot>);

impl Into<Schedule> for Response {
    fn into(self) -> Schedule {
        let mut s: Schedule = Default::default();

        self.0.into_iter()
            .map(|item | {
                let mut slots:Vec<(u8, TimeWindow)> = vec![];
                let tw: TimeWindow = item.times.into();

                if item.plgr_1.free {
                    slots.push((1, tw.clone()))
                }

                if item.plgr_2.free {
                    slots.push((2, tw.clone()))
                }

                if item.plgr_3.free {
                    slots.push((3, tw.clone()))
                }

                if item.plgr_4.free {
                    slots.push((4, tw.clone()))
                }

                if item.plgr_5.free {
                    slots.push((5, tw.clone()))
                }

                slots
            })
            .flatten()
            .for_each(|item| {
                s.entry(item.0).or_default().push(item.1);
            });

        s
    }
}

#[derive(Deserialize)]
pub struct Slot {
    pub times: SlotTime,
    pub plgr_1: Plgr,
    pub plgr_2: Plgr,
    pub plgr_3: Plgr,
    pub plgr_4: Plgr,
    pub plgr_5: Plgr,
}

#[derive(Deserialize)]
pub struct Plgr {
    pub price: String,
    pub free: bool,
    #[serde(deserialize_with = "deserialize_group")]
    pub group: Option<PlgrGroup>,
}

#[derive(Deserialize)]
pub struct PlgrGroup {
    pub group_id: String,
    pub group_time: Option<SlotTime>,
    pub group_duration: u8,
}

#[derive(Deserialize, Clone)]
pub struct SlotTime(pub String);

impl Into<TimeWindow> for SlotTime {
    fn into(self) -> TimeWindow {
        let segments = self.0.split(" - ").collect::<Vec<&str>>();

        let format = format_description!("[hour]:[minute]");

        TimeWindow {
            start: Time::parse(segments.get(0).expect("cant get timeslot.from"), &format).expect("cant parse timeslot.from"),
            end: Time::parse(segments.get(1).expect("cant get timeslot.to"), &format).expect("cant parse timeslot.to"),
        }
    }
}

fn deserialize_group<'de, D>(d: D) -> Result<Option<PlgrGroup>, D::Error>
where
    D: Deserializer<'de>
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Case {
        Target(PlgrGroup),
        EmptyVec([u8; 0]),
    }

    match Case::deserialize(d) {
        Ok(Case::Target(v)) => Ok(Some(v)),
        Ok(Case::EmptyVec(v)) => Ok(None),
        Err(err) => Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let s: Schedule = serde_json::from_str::<Response>(include_str!("../fixtures/spot_slots.json")).expect("should be ok").into();
        println!("{:?}", s);
    }
}