use super::*;

pub mod get_user;

pub fn router_v1(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_user::handler))
        .with_state(state)
}
