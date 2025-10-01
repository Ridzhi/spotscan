use std::collections::HashMap;
use reqwest;
use spotscan::{prelude::*, spot};
use std::ops::Add;
use std::sync::Arc;
use time::{Duration, OffsetDateTime, macros::{format_description, offset}, Weekday};
use tokio;
use log::{error, info, warn};
use grammers_client::{Client as TgClient, InputMessage};
use time::macros::{date, datetime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("SPOT started!");

    let state = Arc::new(AppState::default());
    let bot = factory_bot_client(state.clone()).await;
    let client = reqwest::Client::new();

    loop {
        let spb_time = OffsetDateTime::now_utc().to_offset(offset!(+3:00));

        let dates = (0..8)
            .into_iter()
            .map(|offset| spb_time.add(Duration::days(offset)))
            .collect::<Vec<_>>();

        for date in dates {
            match handler(state.clone(),&bot, &client, date).await {
                Ok(_) => {}
                Err(err) => {
                    error!("handler error: {}", err);
                }
            };
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 10)).await;
    }
}


async fn handler(state: Arc<AppState>, bot: &TgClient, client: &reqwest::Client, date: OffsetDateTime) -> Result<()> {
    let users = state.user_store().find_many(vec![UserOption::Enabled(date.weekday())]).await?;

    if users.is_empty() {
        return Ok(());
    }

    // let schedule = get_schedule(bot, client, date).await?;
    let free_slots = spot::get_free_slots(state.clone(), &date).await?;

    for user in users {
        info!("Handle day {} user {}", date.weekday(), user.tg_user_id);
        let mut matches = vec![];

        free_slots.iter().for_each(|slot| {

        });

        for (playground_number, windows) in &schedule {
            for window in windows {
                if user.match_window(date.weekday(), window) {
                    matches.push((playground_number, window));
                }
            }
        }

        let body = matches.into_iter()
            .map(|(n, window)| {
                create_message(*n, window)
            })
            .collect::<Vec<String>>()
            .join("\n");

        bot.send_message(user, InputMessage::new().text(format!("{}\n{}", get_message_date(&date), body))).await?;
    }

    Ok(())
}

fn create_message(playground_number: u8, w: &TimeWindow) -> String {
    format!(
        "#{} {}-{}",
        playground_number,
        w.start.format(format_description!("[hour]:[minute]")).unwrap(),
        w.end.format(format_description!("[hour]:[minute]")).unwrap(),
    )
}

fn get_message_date(date: &OffsetDateTime) -> String {
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

async fn get_schedule(bot: &TgClient, client: &reqwest::Client, date: OffsetDateTime) -> Result<Vec<(u8, Vec<TimeWindow>)>> {
    let schedule: Schedule = client.get("https://atlanticspot.ru/api/booking/times.php")
        .query(&[("bookingDate", date.format(format_description!("[day].[month].[year]")).expect("date.format failed"))])
        .send()
        .await?
        .json::<spot::Response>()
        .await?
        .into();

    let mut schedule = schedule.into_iter().map(|(k, v)| {
        (k, v)
    })
        .collect::<Vec<(u8, Vec<TimeWindow>)>>();

    schedule.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(schedule)
}