// use signal::domain::Chat as UserChat;
// use std::{env, pin::pin, sync::Arc};
//
// use futures_util::future::{Either, select};
// use grammers_client::{
//     Client, Config, InitParams, Update, grammers_tl_types as tl,
//     session::{PackedType, Session},
//     types::{PackedChat, chat::Chat},
// };
// use deadpool_postgres::Pool;
// use log::{error, info, warn};
//
// use tokio::{runtime, task};
//
// const SESSION_FILE: &str = "bot_main.session";
//
// fn main() -> Result<()> {
//     env_logger::init();
//
//     runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async_main())
// }
//
// async fn handle_update(client: Client, state: Arc<AppState>, update: Update) -> Result<()> {
//     let c = state.config();
//
//     match update {
//         Update::NewMessage(message) if !message.outgoing() && message.text().len() > 0 => {
//             let is_new_message = if let tl::enums::Update::NewMessage(nm) = &message.raw {
//                 matches!(nm.message, tl::enums::Message::Message(_))
//             } else {
//                 false
//             };
//
//             if !is_new_message {
//                 return Ok(());
//             }
//
//             let chat = message.chat();
//
//             if !matches!(chat, Chat::User(_)) {
//                 warn!("main bot: !matches!(chat, Chat::User(_)), got ({:?})", chat);
//
//                 return Ok(());
//             }
//
//             let uc = state.chat_store().find_user_chats(chat.id()).await?;
//             let packed = chat.pack();
//
//             info!("main user chat status = {}", uc.main.is_some());
//             info!("alert user chat status = {}", uc.alert.is_some());
//
//             // logically is the first visit
//             if uc.main.is_none() {
//                 chat::assign_main(state.clone(), &packed).await?;
//                 app::on_first_visit(state.clone(), &packed).await?;
//
//                 message.respond("Welcome!").await?;
//             }
//
//             if uc.alert.is_none() {
//                 chat::assign_alert(state.clone(), &packed).await?;
//             }
//         }
//         _ => {}
//     }
//
//     Ok(())
// }
//
// async fn async_main() -> Result<()> {
//     let state = Arc::new(AppState::default());
//     let config = state.config();
//
//     info!("Main bot starting...");
//     info!("Connecting to Telegram...");
//
//     let client = Client::connect(Config {
//         session: Session::load_file_or_create(SESSION_FILE)?,
//         api_id: config.tg.apiid,
//         api_hash: config.tg.apihash.clone(),
//         params: InitParams {
//             // Fetch the updates we missed while we were offline
//             // catch_up: true,
//             ..Default::default()
//         },
//     })
//         .await?;
//     info!("Connected!");
//
//     if !client.is_authorized().await? {
//         info!("Signing in...");
//
//         client.bot_sign_in(config.tg.bottoken.as_str()).await?;
//
//         client.session().save_to_file(SESSION_FILE)?;
//         info!("Signed in!");
//     }
//
//     info!("Waiting for messages...");
//
//     // This code uses `select` on Ctrl+C to gracefully stop the client and have a chance to
//     // save the session. You could have fancier logic to save the session if you wanted to
//     // (or even save it on every update). Or you could also ignore Ctrl+C and just use
//     // `let update = client.next_update().await?`.
//     //
//     // Using `tokio::select!` would be a lot cleaner but add a heavy dependency,
//     // so a manual `select` is used instead by pinning async blocks by hand.
//     loop {
//         let exit = pin!(async { tokio::signal::ctrl_c().await });
//         let upd = pin!(async { client.next_update().await });
//
//         let update = match select(exit, upd).await {
//             Either::Left(_) => break,
//             Either::Right((u, _)) => u?,
//         };
//
//         let handle = client.clone();
//         let s = state.clone();
//         task::spawn(async move {
//             match handle_update(handle, s, update).await {
//                 Ok(_) => info!("Got update"),
//                 Err(e) => error!("Error handling updates!: {e}"),
//             }
//         });
//     }
//
//     info!("Saving session file and exiting...");
//     client.session().save_to_file(SESSION_FILE)?;
//     Ok(())
// }
//
// pub mod chat {
//     use super::*;
//
//     pub async fn assign_main(state: Arc<AppState>, pc: &PackedChat) -> Result<UserChat> {
//         let bot = match state
//             .bot_store()
//             .find_one(vec![BotOption::Username(&state.config().tg.botusername)])
//             .await?
//         {
//             Some(bot) => bot,
//             None => {
//                 tracing::error!(
//                     "cant found main bot by username({}). check env, migrations",
//                     &state.config().tg.botusername
//                 );
//
//                 return Err(ErrorKind::CantCreateMainUserChat.into());
//             }
//         };
//
//         let chat = UserChat {
//             id: 0,
//             bot_id: bot.id,
//             tg_user_id: pc.id,
//             tg_user_access_hash: pc.access_hash,
//             created_at: UtcDateTime::default(),
//         };
//
//         state.chat_store().save(chat).await
//     }
//
//     pub async fn assign_alert(state: Arc<AppState>, pc: &PackedChat) -> Result<UserChat> {
//         let bot_id = gen_user_bot_id(state.pg().clone(), pc.id).await?;
//         info!("assigned alert bot id {}", bot_id);
//
//         let alert_chat = UserChat {
//             id: 0,
//             bot_id,
//             tg_user_id: pc.id,
//             tg_user_access_hash: None,
//             created_at: UtcDateTime::default(),
//         };
//
//         state.chat_store().save(alert_chat).await
//     }
//
//     async fn gen_user_bot_id(pg: Arc<Pool>, id: TgUserId) -> Result<BotId> {
//         let sql = r#"
//             SELECT
//                 array_agg(id ORDER by id ASC)
//             FROM bot
//             WHERE kind = $1
//             GROUP by kind
//         "#;
//
//         let ids: Vec<BotId> = pg
//             .get()
//             .await?
//             .query_one(sql, &[&BotKind::Alert])
//             .await?
//             .get(0);
//
//         get_user_bot_id(ids, id)
//     }
//
//     fn get_user_bot_id(ids: Vec<BotId>, user_id: TgUserId) -> signal::app::Result<BotId> {
//         if ids.is_empty() {
//             return Err(ErrorKind::CantAssignBotId.into());
//         }
//
//         let ind: i64 = user_id % ids.len() as i64;
//
//         ids.get(ind as usize)
//             .ok_or(ErrorKind::CantAssignBotId.into())
//             .map(|b| *b)
//     }
//
//     #[cfg(test)]
//     mod tests {
//         use super::*;
//         #[test]
//         fn get_user_bot_id_smoke() {
//             let ids = vec![1, 2, 3, 4, 5];
//             assert_eq!(3, get_user_bot_id(ids, 7).unwrap());
//         }
//
//         #[test]
//         fn get_user_bot_id_one_elem() {
//             let ids = vec![1];
//             assert_eq!(1, get_user_bot_id(ids, 7).unwrap());
//         }
//
//         #[test]
//         fn get_user_bot_id_empty() {
//             assert!(get_user_bot_id(vec![], 7).is_err());
//         }
//     }
// }
//
// pub mod app {
//     use super::*;
//
//     pub async fn on_first_visit(state: Arc<AppState>, pc: &PackedChat) -> Result<()> {
//         match state.user_setting_store().find(pc.id).await {
//             Ok(Some(v)) => {
//                 tracing::error!(
//                     "on_first_visit: expect is first visit and not init user settings: {:?}",
//                     v
//                 );
//
//                 Err(ErrorKind::OnFirstVisitUserSettingAlreadyExists.into())
//             }
//             Ok(None) => {
//                 match state
//                     .user_setting_store()
//                     .save(UserSettings::new_default(pc.id))
//                     .await
//                 {
//                     Ok(_) => Ok(()),
//                     Err(e) => {
//                         tracing::error!(
//                             "on_first_visit: first visit for user={}, setting not saved, {:?}",
//                             pc.id,
//                             e
//                         );
//
//                         Err(ErrorKind::OnFirstVisitUserSettingSaveFailed.into())
//                     }
//                 }
//             }
//             Err(e) => {
//                 tracing::error!("on_first_visit: find user setting failed{:?}", e);
//
//                 Err(ErrorKind::OnFirstVisitUserSettingFindFailed.into())
//             }
//         }
//     }
// }

fn main() {}