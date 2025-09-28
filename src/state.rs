pub mod config;
pub mod store;

use self::config::Config;
use deadpool_postgres::Pool;
use std::sync::{Arc, OnceLock};

pub struct AppState {
    config: Factory<Arc<Config>>,
    pg: Factory<Arc<Pool>>,
}

impl AppState {
    pub fn config(&self) -> Arc<Config> {
        self.resolve(&self.config)
    }

    pub fn pg(&self) -> Arc<Pool> {
        self.resolve(&self.pg)
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
