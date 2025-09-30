use reqwest;
use spotscan::{prelude::*, spot};
use std::ops::Add;
use std::sync::Arc;
use time::{
    Duration, OffsetDateTime,
    macros::{format_description, offset},
};
use tokio;
use log::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("SPOT started!");

    let state = Arc::new(AppState::default());
    let client = reqwest::Client::new();

    loop {
        let spb_time = OffsetDateTime::now_utc().to_offset(offset!(+3:00));

        let dates = (0..8)
            .into_iter()
            .map(|offset| spb_time.add(Duration::days(offset)))
            .collect::<Vec<_>>();

        for date in dates {
            handler(state.clone(), &client, date).await;
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


async fn handler(state: Arc<AppState>,client: &reqwest::Client, date: OffsetDateTime) -> Result<()> {
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
        let mut matched = vec![];

        for (playground_number, windows) in &schedule {
            for window in windows {
                if user.match_window(date.weekday(), window) {
                    matched.push((playground_number, window));

                    // warn!("надо слать {} #{} time: {:?}", date.weekday(), playground_number, window);
                }
            }
        }


    }

    Ok(())
}
