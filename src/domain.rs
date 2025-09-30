use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use time::PrimitiveDateTime;
use time::macros::time;
use time::{Duration, Time, Weekday};
use tokio_postgres::types::{FromSql, ToSql};
use utoipa::openapi::{RefOr, Schema, SchemaFormat, schema};
use utoipa::{PartialSchema, ToSchema};

pub type TgUsername = String;
pub type TgUserId = i64;
pub type TgAccessHash = i64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppTime(pub Time);
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

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Settings {
    pub enabled: bool,
    pub app_theme: AppTheme,
    pub defaults: WindowDefaults,
    pub slots: HashMap<AppWeekDay, WindowSettings>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
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
    let now = time::OffsetDateTime::now_utc();

    PrimitiveDateTime::new(now.date(), now.time())
}

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
