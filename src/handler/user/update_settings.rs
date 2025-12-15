use super::*;
use std::collections::HashMap;
use time::Duration;

#[utoipa::path(
    patch,
    path = "/settings",
    operation_id = "update_user_settings",
    responses((status = OK, body = Res)),
    request_body = Req,
)]
pub async fn handler(
    ExtractUser(user): ExtractUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<Req>,
) -> Response {
    let mut user = match state.user_store().find(user.id).await {
        Ok(Some(v)) => v,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return e.into_response(),
    };

    let filters_changed = req.enabled.is_some()
        || req.window_default_duration.is_some()
        || req.window_default_starts.is_some()
        || req.window_default_ends.is_some()
        || req.slots.is_some();

    if filters_changed {
        user.last_slots = None;
    }

    if let Some(v) = req.enabled {
        user.settings.enabled = v;
    }

    if let Some(v) = req.app_theme {
        user.settings.app_theme = v;
    }

    if let Some(v) = req.window_default_duration {
        user.settings.defaults.duration = v;
    }

    if let Some(v) = req.window_default_starts {
        user.settings.defaults.starts = v;
    }

    if let Some(v) = req.window_default_ends {
        user.settings.defaults.ends = v;
    }

    if let Some(v) = req.slots {
        v.into_iter().for_each(|(day, settings)| {
            *user.settings.slots.get_mut(&day).unwrap() = settings;
        });
    }

    match state.user_store().update(user).await {
        Ok(v) => Json(Res { data: v }).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Deserialize, ToSchema)]
#[schema(as = UpdateUserSettingsRequest)]
pub struct Req {
    pub enabled: Option<bool>,
    pub app_theme: Option<AppTheme>,
    pub window_default_duration: Option<Duration>,
    pub window_default_starts: Option<AppTime>,
    pub window_default_ends: Option<AppTime>,
    pub slots: Option<HashMap<AppWeekDay, WindowSettings>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Res {
    pub data: User,
}
