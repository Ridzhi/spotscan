pub mod user;
pub mod club;

use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Json, State},
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};

use init_data_rs as tg;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::prelude::*;

pub struct ExtractUser(TgUser);

impl FromRequestParts<Arc<AppState>> for ExtractUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // @TODO провалидировать решение, возможно лучше через cfg сборки
        if let Some(env) = &state.config().env
            && env == "dev"
        {
            return Ok(ExtractUser(TgUser { id: 140442927 }));
        }

        let authorization_header = match parts.headers.get(header::AUTHORIZATION) {
            Some(hv) => match hv.to_str() {
                Ok(v) => v,
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            },
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        let segments = authorization_header.split_whitespace().collect::<Vec<_>>();

        if segments.clone().len() != 2 {
            return Err(StatusCode::UNAUTHORIZED);
        }

        if segments[0] != "tma" {
            return Err(StatusCode::UNAUTHORIZED);
        }

        match tg::validate(segments[1], &state.config().tg.bottoken, Some(0)) {
            Ok(v) => {
                if let Some(u) = v.user {
                    return Ok(ExtractUser(TgUser { id: u.id }));
                }

                tracing::error!(
                    "tg launch params was validated, but without user({:?})",
                    segments[1]
                );

                Err(StatusCode::UNAUTHORIZED)
            }
            Err(e) => {
                tracing::error!(
                    "invalid tg launch params(err={}), query({:?})",
                    e,
                    segments[1]
                );

                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}
