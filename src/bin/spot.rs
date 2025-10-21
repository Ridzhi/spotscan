use std::collections::HashSet;
use grammers_client::{Client as TgClient, InputMessage};
use log::{error, info};
use spotscan::{prelude::*, spot};
use std::ops::Add;
use std::sync::Arc;
use time::{
    Duration, OffsetDateTime, Weekday,
    macros::{format_description, offset},
};

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

        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 5)).await;
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

    let free_slots = spot::get_free_slots(state.clone(), &date).await?;

    for mut user in users {
        let user_free_slots = free_slots.0.clone()
            .into_iter()
            .filter(|slot| user.match_window(date.weekday(), &slot.window))
            .collect::<Vec<_>>();

        let body = if let Some(v) = &user.last_slots {
            let last = v.iter().collect::<HashSet<_>>();
            let curr = user_free_slots.iter().collect::<HashSet<_>>();

            let booked = last.difference(&curr).collect::<Vec<_>>();
            let freed = curr.difference(&last).collect::<Vec<_>>();

            if booked.is_empty() && freed.is_empty() {
                info!("user{} no updates", &user.tg_user_id);
                continue;
            }

            // actually not show booked slots(need to merge booked for impl),
            user_free_slots
                .iter()
                .map(|slot| {
                    let status: Option<SlotStatus> = if booked.contains(&&slot) {
                        Some(SlotStatus::Booked)
                    } else if freed.contains(&&slot) {
                        Some(SlotStatus::Freed)
                    } else {
                        None
                    };

                    create_message(slot.field, &slot.window, status)
                })
                .collect::<Vec<String>>()

        } else {
            user_free_slots
                .iter()
                .map(|slot| create_message(slot.field, &slot.window, None))
                .collect::<Vec<String>>()
        };

        user.last_slots = Some(Slots(user_free_slots));
        user = state.user_store().update(user).await?;

        if body.is_empty() {
            info!("no available slots for user {}, skip", &user.tg_user_id);
            continue;
        }

        let tg_message = format!("{}\n{}", get_message_date(&date), body.join("\n"));

        match bot.send_message(
            user,
            InputMessage::new().text(tg_message.clone()),
        )
        .await {
            Ok(_) => {}
            Err(e) => {
                error!("bot.send_message: {}, message {}", e, tg_message);
            }
        }
    }

    Ok(())
}

fn create_message(f: FieldNumber, w: &TimeWindow, status: Option<SlotStatus>) -> String {
    let m = format!(
        "{}-{} #{} ",
        w.start
            .format(format_description!("[hour]:[minute]"))
            .unwrap(),
        w.end
            .format(format_description!("[hour]:[minute]"))
            .unwrap(),
        f,
    );

    match status {
        None => m,
        Some(SlotStatus::Booked) => {
            format!("{} (заняли)", m)
        },
        Some(SlotStatus::Freed) => {
            format!("{} (освободили)", m)
        },
    }
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
