use spotscan::prelude::*;
use std::{pin::pin, sync::Arc};
use futures_util::future::{Either, select};
use grammers_client::{
    Client, Config, InitParams, Update, grammers_tl_types as tl,
    session::{Session},
    types::{PackedChat, chat::Chat},
};
use log::{error, info, warn};

use tokio::{runtime, task};

const SESSION_FILE: &str = "bot.session";

fn main() -> Result<()> {
    env_logger::init();
    
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn handle_update(_: Client, state: Arc<AppState>, update: Update) -> Result<()> {
    match update {
        Update::NewMessage(message) if !message.outgoing() && !message.text().is_empty() => {
            let is_new_message = if let tl::enums::Update::NewMessage(nm) = &message.raw {
                matches!(nm.message, tl::enums::Message::Message(_))
            } else {
                false
            };

            if !is_new_message {
                return Ok(());
            }

            let chat = message.chat();
            let packed = chat.pack();

            if !matches!(chat, Chat::User(_)) {
                warn!("main bot: !matches!(chat, Chat::User(_)), got ({:?})", chat);

                return Ok(());
            }

            match state.user_store().find(chat.id()).await? {
                Some(v) => v,
                None => {
                    message.respond("Кажется это ваш первый визит. Здесь можно смотреть свободные слоты клуба SPOT (https://atlanticspot.ru) по заданным фильтрам а также настроить бота, который будет следить за обновлениями свободных слотов с учетом ваших фильтров").await?;

                    info!("first visit for id={}", chat.id());
                    let u = setup_user(state, &packed).await?;
                    info!("id={} successfully setup", chat.id());
                    u
                }
            };
        }
        _ => {}
    }

    Ok(())
}

async fn async_main() -> Result<()> {
    let state = Arc::new(AppState::default());
    let config = state.config();

    info!("Main bot starting...");
    info!("Connecting to Telegram...");

    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id: config.tg.apiid,
        api_hash: config.tg.apihash.clone(),
        params: InitParams {
            // Fetch the updates we missed while we were offline
            // catch_up: true,
            ..Default::default()
        },
    })
    .await?;
    info!("Connected!");

    if !client.is_authorized().await? {
        info!("Signing in...");

        client.bot_sign_in(config.tg.bottoken.as_str()).await?;

        client.session().save_to_file(SESSION_FILE)?;
        info!("Signed in!");
    }

    info!("Waiting for messages...");

    // This code uses `select` on Ctrl+C to gracefully stop the client and have a chance to
    // save the session. You could have fancier logic to save the session if you wanted to
    // (or even save it on every update). Or you could also ignore Ctrl+C and just use
    // `let update = client.next_update().await?`.
    //
    // Using `tokio::select!` would be a lot cleaner but add a heavy dependency,
    // so a manual `select` is used instead by pinning async blocks by hand.
    loop {
        let exit = pin!(async { tokio::signal::ctrl_c().await });
        let upd = pin!(async { client.next_update().await });

        let update = match select(exit, upd).await {
            Either::Left(_) => break,
            Either::Right((u, _)) => u?,
        };

        let handle = client.clone();
        let s = state.clone();
        task::spawn(async move {
            match handle_update(handle, s, update).await {
                Ok(_) => info!("Got update"),
                Err(e) => error!("Error handling updates!: {e}"),
            }
        });
    }

    info!("Saving session file and exiting...");
    client.session().save_to_file(SESSION_FILE)?;
    Ok(())
}

async fn setup_user(state: Arc<AppState>, pc: &PackedChat) -> Result<User> {
    match state
        .user_store()
        .save(User::new(pc.id, pc.access_hash.unwrap()))
        .await
    {
        Ok(v) => Ok(v),
        Err(e) => {
            tracing::error!(
                "on_first_visit: first visit for user={}, user not saved, {:?}",
                pc.id,
                e
            );

            Err(ErrorKind::OnFirstVisitUserSaveFailed.into())
        }
    }
}
