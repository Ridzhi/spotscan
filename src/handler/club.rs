use super::*;

mod get_user_slots;

pub fn router_v1(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_user_slots::handler))
        .with_state(state)
}
