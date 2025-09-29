use super::*;

#[utoipa::path(
    get,
    path = "/",
    operation_id = "get_user",
    responses((status = OK, body = Res))
)]
pub async fn handler(
    ExtractUser(user): ExtractUser,
    State(state): State<Arc<AppState>>,
) -> Response {
    match state.user_store().find(user.id).await {
        Ok(Some(v)) => Json(Res { data: v }).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Res {
    pub data: User,
}
