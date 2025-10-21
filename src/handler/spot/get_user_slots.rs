use super::*;

#[utoipa::path(
    get,
    path = "/slots",
    operation_id = "get_user_slots",
    responses((status = OK, body = Res))
)]
pub async fn handler(
    ExtractUser(user): ExtractUser,
    State(state): State<Arc<AppState>>,
) -> Response {
    match state.user_store().find(user.id).await {
        Ok(Some(u)) => Json(Res {
            data: spot::get_user_free_slots(state.clone(), &u)
                .await
                .expect("get user free slots"),
        })
        .into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => err.into_response(),
    }
}

#[derive(Serialize, ToSchema)]
#[schema(as = GetUserSlots)]
pub struct Res {
    pub data: SlotsWeek,
}
