use std::time::Duration;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::json;
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{error::AppError, handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { Json(json!({"status": "OK"})) }))
        .route("/health", get(|| async { StatusCode::OK }))
        .route(
            "/todos",
            post(handlers::create_todo).get(handlers::get_todos),
        )
        .route(
            "/todos/{id}",
            get(handlers::get_todo)
                .put(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .fallback(|| async { AppError::NotFound })
        .layer(RequestBodyLimitLayer::new(64 * 1024))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
