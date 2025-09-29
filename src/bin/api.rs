use std::fs;
use std::sync::Arc;

use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use axum::{routing::get};
use utoipa::{OpenApi};
use utoipa_axum::{router::{OpenApiRouter}};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use spotscan::{handler, prelude::*};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "spotscan", license(identifier = "GPL")),

    )]
    struct ApiDoc;

    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState::default());

    if state.pg().get().await.is_err() {
        panic!("Failed to connect to database");
    }

    let config = state.config().clone();

    // build our application with a single route
    let (app, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/ping", get(async || { "pong"}))
        .nest("/v1/user", handler::user::router_v1(state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .split_for_parts();

    let _ = fs::write("./docs/openapi.json", api.to_pretty_json().unwrap()) ;

    let app = app.merge(Scalar::with_url("/doc/api", api));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
