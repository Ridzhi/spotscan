use time::OffsetDateTime;
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
                .expect("get user free slots")
                .into(),
        })
        .into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => err.into_response(),
    }
}

#[derive(Serialize, ToSchema)]
#[schema(as = GetUserSlots)]
pub struct Res {
    pub data: ResSlots,
}

#[derive(Serialize, ToSchema)]
pub struct ResSlots(pub Vec<ResDaySlot>);

#[derive(Serialize, ToSchema)]
pub struct ResDaySlot{
    pub date: String,
    pub windows: Vec<ResWindow>
}
#[derive(Serialize, ToSchema)]
pub struct ResWindow{
    pub field: FieldNumber,
    pub window: TimeWindow
}
impl Into<ResSlots> for FreeSlots {
    fn into(self) -> ResSlots {
        ResSlots(self.0.into_iter()
            .map(|d| {
                let mut windows: Vec<ResWindow> = d.slots.0.into_iter()
                    .map(|item| {
                        item.windows.iter()
                            .map(|tw| {
                                ResWindow{
                                    field: item.field,
                                    window: tw.clone(),
                                }
                            })
                            .collect::<Vec<ResWindow>>()
                    })
                    .flatten()
                    .collect();

                windows.sort_by(|a, b| a.window.start.cmp(&b.window.start));

                ResDaySlot {
                    date: get_human_day(&d.date),
                    windows
                }
            })
            .collect()
        )
    }
}