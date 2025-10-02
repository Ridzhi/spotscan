use grammers_client::{Client as TgClient, InputMessage};
use log::{error, info};
use reqwest;
use spotscan::{prelude::*, spot};
use std::ops::Add;
use std::sync::Arc;
use time::{
    Duration, OffsetDateTime, Weekday,
    macros::{format_description, offset},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("SPOT started!");

    let state = Arc::new(AppState::default());
    let bot = factory_bot_client(state.clone()).await;

    loop {
        let spb_time = OffsetDateTime::now_utc().to_offset(offset!(+3:00));

        let dates = (0..8)
            .into_iter()
            .map(|offset| spb_time.add(Duration::days(offset)))
            .collect::<Vec<_>>();

        for date in dates {
            match handler(state.clone(), &bot, date).await {
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

async fn handler(
    state: Arc<AppState>,
    bot: &TgClient,
    date: OffsetDateTime,
) -> Result<()> {
    let users = state
        .user_store()
        .find_many(vec![UserOption::Enabled(date.weekday())])
        .await?;

    if users.is_empty() {
        return Ok(());
    }

    info!("expect handle users count = {} for date = {}", users.len(), &date);

    let free_slots = spot::get_free_slots(state.clone(), &date).await?;

    for user in users {
        info!("Handle day {} user {}", date.weekday(), user.tg_user_id);

        let body = free_slots
            .iter()
            .filter(|slot| user.match_window(date.weekday(), &slot.window))
            .map(|slot| create_message(slot.field, &slot.window))
            .collect::<Vec<String>>()
            .join("\n");

        bot.send_message(
            user,
            InputMessage::new().text(format!("{}\n{}", get_message_date(&date), body)),
        )
        .await?;
    }

    Ok(())
}

fn create_message(f: FieldNumber, w: &TimeWindow) -> String {
    format!(
        "{}-{} #{} ",
        w.start
            .format(format_description!("[hour]:[minute]"))
            .unwrap(),
        w.end
            .format(format_description!("[hour]:[minute]"))
            .unwrap(),
        f,
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
        Weekday::Sunday => "Вск",
    };

    format!("{}, {}", weekday, date.day())
}
