use self::config::Config;
use deadpool_postgres::Pool;
use std::sync::{Arc, OnceLock};
use grammers_client::{Client as TgClient, InitParams};
use grammers_client::session::Session;
use store::*;

pub mod config;
pub mod store;

pub struct AppState {
    config: Factory<Arc<Config>>,
    pg: Factory<Arc<Pool>>,
    user_store: Factory<Arc<UserStore>>,
}

impl AppState {
    pub fn config(&self) -> Arc<Config> {
        self.resolve(&self.config)
    }

    pub fn pg(&self) -> Arc<Pool> {
        self.resolve(&self.pg)
    }

    pub fn user_store(&self) -> Arc<UserStore> {
        self.resolve(&self.user_store)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Factory::once(|_| Arc::new(Config::from_env())),
            pg: Factory::once(|state| {
                let pg = &state.config().pg;
                let pool = pg
                    .create_pool(Some(deadpool::Runtime::Tokio1), tokio_postgres::NoTls)
                    .unwrap();

                Arc::new(pool)
            }),
            user_store: Factory::once(|state| Arc::new(UserStore::new(state.pg()))),
        }
    }
}

impl AppState {
    fn resolve<T>(&self, factory: &Factory<T>) -> T
    where
        T: Clone,
    {
        (factory.0)(self)
    }
}

pub async fn factory_bot_client(state: Arc<AppState>) -> TgClient {
    let session_file = "bot.session";

    let client = TgClient::connect(grammers_client::Config {
        session: Session::load_file_or_create(session_file).expect("bot cant load session file"),
        api_id: state.config().tg.apiid,
        api_hash: state.config().tg.apihash.clone(),
        params: InitParams {
            ..Default::default()
        },
    })
        .await.expect("bot cant connect");


    if !client.is_authorized().await.expect("client.is_authorized()") {
        client.bot_sign_in(state.config().tg.bottoken.as_str()).await.expect("client.bot_sign_in");

        client.session().save_to_file(session_file).expect("bot cant save session file");
    }

    client
}

#[derive(Clone)]
struct Factory<T>(Arc<dyn Fn(&AppState) -> T + Send + Sync>);

impl<T> Factory<T> {
    pub fn once(f: impl Fn(&AppState) -> T + Send + Sync + 'static) -> Self
    where
        T: Send + Sync + Clone + 'static,
    {
        let cell = OnceLock::new();
        Factory(Arc::new(move |s| cell.get_or_init(|| f(s)).clone()))
    }
}
