use reqwest;
use spotscan::{prelude::*, spot};
use std::ops::Add;
use std::sync::Arc;
use time::{Duration, OffsetDateTime, macros::{format_description, offset}, Weekday};
use tokio;
use log::{error, info, warn};
use grammers_client::{Client as TgClient, InputMessage};

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
            handler(state.clone(),&bot, &client, date).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }

    // берем каждую дату
    // делаем запрос
    // обрабатываем данные
    // спим 5 сек

    // спим минут после пачки дат

    // println!("Spot: {:?}", dates);

    // let client = reqwest::Client::new();
    // let schedule: Schedule = client.get("https://atlanticspot.ru/api/booking/times.php")
    //     .query(&[("bookingDate", "30.09.2025")])
    //     .send()
    //     .await?
    //     .json::<spot::Response>()
    //     .await?
    //     .into();
    //
    // println!("{schedule:#?}");
    Ok(())
}


async fn handler(state: Arc<AppState>, bot: &TgClient, client: &reqwest::Client, date: OffsetDateTime) -> Result<()> {
    info!("Handle {}", date.weekday());

    let users = state.user_store().find_many(vec![UserOption::Enabled(date.weekday())]).await?;

    if users.is_empty() {
        return Ok(());
    }

    let schedule: Schedule = client.get("https://atlanticspot.ru/api/booking/times.php")
        .query(&[("bookingDate", date.format(format_description!("[day].[month].[year]")).expect("date.format failed"))])
        .send()
        .await?
        .json::<spot::Response>()
        .await?
        .into();

    for user in users {
        let mut matches = vec![];

        for (playground_number, windows) in &schedule {
            for window in windows {
                if user.match_window(date.weekday(), window) {
                    matches.push((playground_number, window));
                }
            }
        }

        let msg = matches.into_iter()
            .map(|(n, window)| {
                create_message(&date, *n, window)
            })
            .collect::<Vec<String>>()
            .join("\n");

        bot.send_message(user, InputMessage::new().text(msg)).await?;
    }

    Ok(())
}

fn create_message(date: &OffsetDateTime, playground_number: u8, w: &TimeWindow) -> String {
    let weekday = match date.weekday() {
        Weekday::Monday => "Пн",
        Weekday::Tuesday => "Вт",
        Weekday::Wednesday => "Ср",
        Weekday::Thursday => "Чт",
        Weekday::Friday => "Пт",
        Weekday::Saturday => "Сб",
        Weekday::Sunday => "Вск"
    };

    format!(
        "{},{}: #{} {}-{}",
        weekday,
        date.day(),
        playground_number,
        w.start.format(format_description!("[hour]:[minute]")).unwrap(),
        w.end.format(format_description!("[hour]:[minute]")).unwrap(),
    )
}
