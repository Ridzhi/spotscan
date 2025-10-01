use super::*;
use crate::spot;

mod get_schedule;

pub fn router_v1(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_schedule::handler))
        .with_state(state)
}