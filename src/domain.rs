use grammers_client::session::PackedType;
use grammers_client::types::PackedChat;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Add, Deref, DerefMut};
use time::macros::{offset, time};
use time::{Duration, Time, Weekday};
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio_postgres::types::{FromSql, ToSql};
use utoipa::openapi::{RefOr, Schema, SchemaFormat, schema};
use utoipa::{PartialSchema, ToSchema};

pub type TgUsername = String;
pub type TgUserId = i64;
pub type TgAccessHash = i64;
pub type FieldNumber = u8;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
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
    pub last_slots: Option<Slots>,
    pub settings: Settings,
    pub created_at: UtcDateTime,
}

impl User {
    pub fn match_window(&self, d: Weekday, w: &TimeWindow) -> bool {
        let user_starts = self.settings.get_starts(d).0;
        let user_ends = self.settings.get_ends(d).0;

        if w.fixed && (user_starts.gt(&w.start.0) || user_ends.lt(&w.end.0)) {
            return false;
        }

        let from = std::cmp::max(user_starts, w.start.0);
        let to = std::cmp::min(user_ends, w.end.0);

        if from >= to {
            return false;
        }

        self.settings.get_duration(d).le(&from.duration_until(to))
    }
}

impl User {
    pub fn new(tg_user_id: TgUserId, tg_user_access_hash: TgAccessHash) -> Self {
        Self {
            id: 0,
            tg_user_id,
            tg_user_access_hash,
            settings: Settings::default(),
            last_slots: None,
            created_at: Default::default(),
        }
    }
}

impl From<User> for PackedChat {
    fn from(value: User) -> Self {
        PackedChat {
            ty: PackedType::User,
            id: value.tg_user_id,
            access_hash: value.tg_user_access_hash.into(),
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct UtcDateTime(pub PrimitiveDateTime);

impl Default for UtcDateTime {
    fn default() -> Self {
        Self(now_utc())
    }
}

impl From<UtcDateTime> for PrimitiveDateTime {
    fn from(value: UtcDateTime) -> Self {
        value.0
    }
}

pub fn now_utc() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();

    PrimitiveDateTime::new(now.date(), now.time())
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, ToSchema)]
pub struct Slots(pub Vec<Slot>);

impl Deref for Slots {
    type Target = Vec<Slot>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq, ToSchema)]
pub struct Slot {
    pub field: FieldNumber,
    pub window: TimeWindow,
}

#[derive(Serialize, Deserialize, Default, Debug, ToSchema)]
pub struct SlotsWeek(pub Vec<FreeSlotWeek>);

pub enum SlotStatus {
    Booked,
    Freed,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FreeSlotWeek {
    pub date: OffsetDateTime,
    pub slots: Slots,
}

impl Deref for SlotsWeek {
    type Target = Vec<FreeSlotWeek>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SlotsWeek {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct DateIter {
    now: OffsetDateTime,
    start: i64,
    end: i64,
}

impl DateIter {
    pub fn new() -> Self {
        // lookup one week ahead
        Self {
            now: OffsetDateTime::now_utc().to_offset(offset!(+3:00)),
            start: 0,
            end: 15,
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

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug, ToSchema)]
pub struct TimeWindow {
    pub fixed: bool,
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
