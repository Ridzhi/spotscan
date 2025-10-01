use crate::prelude::*;
use log::info;
use serde::{Deserialize, de::Deserializer};
use std::collections::HashMap;
use std::sync::Arc;
use time::{OffsetDateTime, Time, macros::format_description};
use tokio_postgres::fallible_iterator::FallibleIterator;

static URL: &str = "https://atlanticspot.ru/api/booking/times.php";

#[derive(Deserialize)]
pub struct Response(pub Vec<Slot>);

impl Into<FreeSlotsDay> for Response {
    fn into(self) -> FreeSlotsDay {
        let mut s: HashMap<FieldNumber, Vec<TimeWindow>> = Default::default();

        self.0
            .into_iter()
            .map(|item| {
                let mut slots: Vec<(u8, TimeWindow)> = vec![];
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
                let entry = s.entry(item.0).or_default();

                if let Some(v) = entry.last_mut() {
                    if item.1.start.eq(&v.end) {
                        v.end = item.1.end;
                        return;
                    }
                }
                entry.push(item.1);
            });

        FreeSlotsDay(
            s.into_iter()
                .map(|item| {
                    FieldSlot{
                        field: item.0,
                        windows: item.1,
                    }
                })
                .collect(),
        )
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
            start: Time::parse(segments.get(0).expect("cant get timeslot.from"), &format)
                .expect("cant parse timeslot.from")
                .into(),
            end: Time::parse(segments.get(1).expect("cant get timeslot.to"), &format)
                .expect("cant parse timeslot.to")
                .into(),
        }
    }
}

fn deserialize_group<'de, D>(d: D) -> Result<Option<PlgrGroup>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Case {
        Target(PlgrGroup),
        EmptyVec([u8; 0]),
    }

    match Case::deserialize(d) {
        Ok(Case::Target(v)) => Ok(Some(v)),
        Ok(Case::EmptyVec(_)) => Ok(None),
        Err(err) => Err(err),
    }
}

pub async fn get_user_free_slots(state: Arc<AppState>, user: &User) -> Result<FreeSlots> {
    let dates = DateIter::new().into_iter().filter(|d| {
        user.settings
            .slots
            .get(&d.weekday().into())
            .unwrap()
            .enabled
    });

    let mut result: FreeSlots = Default::default();

    for date in dates {
        info!(
            "Getting user({}) free slots: date={:?}",
            user.tg_user_id, date
        );
        
        let user_free_slots = get_free_slots(state.clone(), &date)
            .await?
            .0
            .into_iter()
            .map(|slot| {
                FieldSlot {
                    field: slot.field,
                    windows: slot.windows.into_iter()
                        .filter(|w| {
                            user.match_window(date.weekday(), w)
                        })
                        .collect(),
                }
            })
            .collect::<Vec<FieldSlot>>();

        result.0.push(DaySlot{
            date,
            slots: FreeSlotsDay(user_free_slots)
        });
    }

    Ok(result)
}

pub async fn get_free_slots(state: Arc<AppState>, date: &OffsetDateTime) -> Result<FreeSlotsDay> {
    let mut s: FreeSlotsDay = state
        .http_client()
        .get(URL)
        .query(&[(
            "bookingDate",
            date.format(format_description!("[day].[month].[year]"))
                .expect("date.format failed"),
        )])
        .send()
        .await?
        .json::<Response>()
        .await?
        .into();

    s.sort_by(|a, b| a.field.cmp(&b.field));

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let s: FreeSlotsDay =
            serde_json::from_str::<Response>(include_str!("../fixtures/spot_slots.json"))
                .expect("should be ok")
                .into();
        println!("{:?}", s);
    }
}
