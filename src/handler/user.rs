use super::*;

mod get_user;
mod update_settings;

pub fn router_v1(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_user::handler))
        .routes(routes!(update_settings::handler))
        .with_state(state)
}
