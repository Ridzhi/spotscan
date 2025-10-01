use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Add, Deref, DerefMut};
use grammers_client::session::PackedType;
use grammers_client::types::PackedChat;
use time::{OffsetDateTime, PrimitiveDateTime};
use time::macros::{offset, time};
use time::{Duration, Time, Weekday};
use tokio_postgres::types::{FromSql, ToSql};
use utoipa::openapi::{RefOr, Schema, SchemaFormat, schema};
use utoipa::{PartialSchema, ToSchema};

pub type TgUsername = String;
pub type TgUserId = i64;
pub type TgAccessHash = i64;
pub type FieldNumber = u8;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppTime(pub Time);

impl From<Time> for AppTime {
    fn from(t: Time) -> AppTime {
        AppTime(t)
    }
}

impl Deref for AppTime {
    type Target = Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AppWeekDay(pub Weekday);

impl From<Weekday> for AppWeekDay {
    fn from(weekday: Weekday) -> Self {
        Self(weekday)
    }
}

pub struct TgUser {
    pub id: TgUserId,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct User {
    pub id: i64,
    pub tg_user_id: TgUserId,
    pub tg_user_access_hash: TgAccessHash,
    pub settings: Settings,
    pub created_at: UtcDateTime,
}

impl User {
    pub fn match_window(&self, d: Weekday, w: &TimeWindow) -> bool {
        self.settings.get_starts(d).le(&w.start)
        && self.settings.get_ends(d).ge(&w.end)
        && self
            .settings
            .get_duration(d)
            .le(&w.start.duration_until(w.end.0))
    }
}

impl User {
    pub fn new(tg_user_id: TgUserId, tg_user_access_hash: TgAccessHash) -> Self {
        Self {
            id: 0,
            tg_user_id,
            tg_user_access_hash,
            settings: Settings::default(),
            created_at: Default::default(),
        }
    }
}

impl Into<PackedChat> for User {
    fn into(self) -> PackedChat {
        PackedChat {
            ty: PackedType::User,
            id: self.tg_user_id,
            access_hash: self.tg_user_access_hash.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Settings {
    pub enabled: bool,
    pub app_theme: AppTheme,
    pub defaults: WindowDefaults,
    pub slots: HashMap<AppWeekDay, WindowSettings>,
}

impl Settings {
    pub fn get_duration(&self, day: Weekday) -> Duration {
        self.slots
            .get(&day.into())
            .expect("day.weekday out of bounds")
            .duration
            .unwrap_or(self.defaults.duration)
    }

    pub fn get_starts(&self, day: Weekday) -> AppTime {
        self.slots
            .get(&day.into())
            .expect("day.weekday out of bounds")
            .starts
            .clone()
            .unwrap_or(self.defaults.starts.clone())
    }

    pub fn get_ends(&self, day: Weekday) -> AppTime {
        self.slots
            .get(&day.into())
            .expect("day.weekday out of bounds")
            .ends
            .clone()
            .unwrap_or(self.defaults.ends.clone())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            app_theme: AppTheme::System,
            defaults: WindowDefaults {
                duration: Duration::minutes(120),
                starts: AppTime(time!(10:00)),
                ends: AppTime(time!(20:00)),
            },
            slots: HashMap::from([
                (Weekday::Monday.into(), WindowSettings::default()),
                (Weekday::Tuesday.into(), WindowSettings::default()),
                (Weekday::Wednesday.into(), WindowSettings::default()),
                (Weekday::Thursday.into(), WindowSettings::default()),
                (Weekday::Friday.into(), WindowSettings::default()),
                (
                    Weekday::Saturday.into(),
                    WindowSettings {
                        enabled: true,
                        ..WindowSettings::default()
                    },
                ),
                (
                    Weekday::Sunday.into(),
                    WindowSettings {
                        enabled: true,
                        ..WindowSettings::default()
                    },
                ),
            ]),
        }
    }
}

pub enum UserOption {
    TgUserId(TgUserId),
    Enabled(Weekday),
}

pub type UserOptions = Vec<UserOption>;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct WindowDefaults {
    pub duration: Duration,
    pub starts: AppTime,
    pub ends: AppTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, Default)]
pub struct WindowSettings {
    pub enabled: bool,
    pub duration: Option<Duration>,
    pub starts: Option<AppTime>,
    pub ends: Option<AppTime>,
}
// слот должен проходить проверку на:
//  - начинается не раньше указанной границы
//  - заканчивается не позже указанной границы
//  - имеет длительность не меньше заданной

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct UtcDateTime(pub PrimitiveDateTime);

impl Default for UtcDateTime {
    fn default() -> Self {
        Self(now_utc())
    }
}

impl Into<PrimitiveDateTime> for UtcDateTime {
    fn into(self) -> PrimitiveDateTime {
        self.0
    }
}

pub fn now_utc() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();

    PrimitiveDateTime::new(now.date(), now.time())
}

#[derive(Serialize, Deserialize, Default, Debug, ToSchema)]
pub struct FreeSlotsDay(pub Vec<FieldSlot>);

#[derive(Serialize, Deserialize, Default, Debug, ToSchema)]
pub struct FieldSlot {
    pub field: FieldNumber,
    pub windows: Vec<TimeWindow>,
}

#[derive(Serialize, Deserialize, Default, Debug ,ToSchema)]
pub struct FreeSlots(pub Vec<DaySlot>);

#[derive(Serialize, Deserialize, Debug ,ToSchema)]
pub struct DaySlot {
    pub date: OffsetDateTime,
    pub slots: FreeSlotsDay,
}

impl Deref for FreeSlotsDay {
    type Target = Vec<FieldSlot>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FreeSlotsDay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type Schedule = HashMap<FieldNumber, Vec<TimeWindow>>;

pub struct DateIter {
    now: OffsetDateTime,
    start: i64,
    end: i64,
}

impl DateIter {
    pub fn new() -> Self {
        // lookup one week ahead
        Self{
            now: OffsetDateTime::now_utc().to_offset(offset!(+3:00)),
            start: 0,
            end: 8,
        }
    }
}

impl Default for DateIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for DateIter {
    type Item = OffsetDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let result = self.now.add(Duration::days(self.start));
        self.start += 1;
        Some(result)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct TimeWindow {
    pub start: AppTime,
    pub end: AppTime,
}

impl PartialSchema for AppTime {
    fn schema() -> RefOr<Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(schema::Type::String)
            .format(Some(SchemaFormat::Custom("[hour]:[minute]".to_string())))
            .into()
    }
}

impl ToSchema for AppTime {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Time")
    }
}

impl PartialSchema for AppWeekDay {
    fn schema() -> RefOr<Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(schema::Type::String)
            .format(Some(SchemaFormat::Custom("time::Weekday".to_string())))
            .into()
    }
}

impl ToSchema for AppWeekDay {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Weekday")
    }
}

#[derive(
    Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq, FromSql, ToSql, strum::Display,
)]
#[postgres(name = "app_theme")]
pub enum AppTheme {
    #[postgres(name = "LIGHT")]
    #[strum(to_string = "LIGHT")]
    Light,

    #[postgres(name = "DARK")]
    #[strum(to_string = "DARK")]
    Dark,

    #[postgres(name = "SYSTEM")]
    #[strum(to_string = "SYSTEM")]
    System,
}

pub fn get_human_day(date: &OffsetDateTime) -> String {
    let weekday = match date.weekday() {
        Weekday::Monday => "Пн",
        Weekday::Tuesday => "Вт",
        Weekday::Wednesday => "Ср",
        Weekday::Thursday => "Чт",
        Weekday::Friday => "Пт",
        Weekday::Saturday => "Сб",
        Weekday::Sunday => "Вск"
    };

    format!("{},{}", weekday, date.day())
}