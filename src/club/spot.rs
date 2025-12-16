use crate::prelude::*;
use log::info;
use serde::{de::Deserializer, Deserialize};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};
use time::{macros::format_description, OffsetDateTime, Time};
use crate::app::HttpGet;


static URL: &str = "https://atlanticspot.ru/api/booking/times.php";

type GroupId = String;

#[derive(Deserialize)]
pub struct Response(pub Vec<SSlot>);

impl From<Response> for Slots {
    fn from(response: Response) -> Slots {
        let mut s: HashMap<FieldNumber, Vec<TimeWindow>> = Default::default();
        let mut groups: HashSet<GroupId> = HashSet::new();

        response
            .0
            .into_iter()
            .flat_map(|item| {
                let mut slots: Vec<(FieldNumber, TimeWindow)> = vec![];
                let tw: TimeWindow = item.times.clone().into_time_window(false);

                for (i, field) in item.fields().iter().enumerate() {
                    if !field.free {
                        continue;
                    }

                    match &field.group {
                        None => slots.push(((i + 1) as FieldNumber, tw.clone())),
                        Some(v) => {
                            if groups.contains(&v.group_id) {
                                continue;
                            }

                            if let Some(t) = &v.group_time {
                                // it's possible because human can wrong while setup schedule
                                if t.is_empty() {
                                    continue;
                                }

                                groups.insert(v.group_id.clone());

                                let gt: TimeWindow = t.clone().into_time_window(true);
                                slots.push(((i + 1) as FieldNumber, gt.clone()));
                            }
                        }
                    }
                }

                slots
            })
            .for_each(|item| {
                let entry = s.entry(item.0).or_default();

                if let Some(v) = entry.last_mut()
                    && !item.1.fixed
                    && !v.fixed
                    && item.1.start.eq(&v.end)
                {
                    v.end = item.1.end;
                    return;
                }
                entry.push(item.1);
            });

        Slots(
            s.into_iter()
                .flat_map(|item| {
                    item.1
                        .into_iter()
                        .map(|tw| Slot {
                            field: item.0,
                            window: tw,
                        })
                        .collect::<Vec<Slot>>()
                })
                .collect(),
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SSlot {
    pub times: SlotTime,
    pub plgr_1: Plgr,
    pub plgr_2: Plgr,
    pub plgr_3: Plgr,
    pub plgr_4: Plgr,
    pub plgr_5: Plgr,
}

impl SSlot {
    pub fn fields(&self) -> [&Plgr; 5] {
        [
            &self.plgr_1,
            &self.plgr_2,
            &self.plgr_3,
            &self.plgr_4,
            &self.plgr_5,
        ]
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Plgr {
    pub price: String,
    pub free: bool,
    #[serde(deserialize_with = "deserialize_group")]
    pub group: Option<PlgrGroup>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PlgrGroup {
    pub group_id: String,
    pub group_time: Option<SlotTime>,
    pub group_duration: u8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SlotTime(pub String);

impl Deref for SlotTime {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SlotTime {
    fn into_time_window(self, fixed: bool) -> TimeWindow {
        let segments = self.0.split(" - ").collect::<Vec<&str>>();

        let format = format_description!("[hour]:[minute]");

        TimeWindow {
            fixed,
            #[allow(clippy::get_first)]
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

pub async fn get_user_free_slots<C>(client: Arc<C>, user: &User) -> Result<SlotsWeek>
where
    C: HttpGet {
    let dates = DateIter::new().filter(|d| {
        user.settings
            .slots
            .get(&d.weekday().into())
            .unwrap()
            .enabled
    });

    let mut result: SlotsWeek = Default::default();

    for date in dates {
        info!(
            "Getting user({}) free slots: date={:?}",
            user.tg_user_id, date
        );

        let mut user_free_slots = get_free_slots_v2(client.clone(), &date)
            .await?
            .0
            .into_iter()
            .filter(|slot| user.match_window(date.weekday(), &slot.window))
            .collect::<Vec<Slot>>();

        if user_free_slots.is_empty() {
            continue;
        }

        user_free_slots.sort_by(|a, b| a.window.start.cmp(&b.window.start));

        result.0.push(FreeSlotWeek {
            date,
            slots: Slots(user_free_slots),
        });
    }

    Ok(result)
}

pub async fn get_free_slots(state: Arc<AppState>, date: &OffsetDateTime) -> Result<Slots> {
    let mut s: Slots = state
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

    s.0.sort_by(|a, b| a.field.cmp(&b.field));

    Ok(s)
}

pub async fn get_free_slots_v2<C: HttpGet>(client: Arc<C>, date: &OffsetDateTime) -> Result<Slots> {
    let mut s: Slots = client
        .get(
            URL,
            &[(
                "bookingDate",
                date.format(format_description!("[day].[month].[year]"))
                    .expect("date.format failed"),
            )],
        )
        .await?
        .json::<Response>()
        .await?
        .into();

    s.0.sort_by(|a, b| a.field.cmp(&b.field));

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn parse() {
        let s: Slots = serde_json::from_str::<Response>(include_str!("../../fixtures/failed.json"))
            .expect("should be ok")
            .into();
        println!("{:?}", s);

        let mut file =
            File::create("../../fixtures/free_slots_merged.json").expect("create file failed");

        file.write_all(serde_json::to_string(&s).unwrap().as_bytes());
    }
}
