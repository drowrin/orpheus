use axum::{http::StatusCode, response::IntoResponse, routing, Router};

use crate::state::AppState;

pub async fn projects() -> impl IntoResponse {
    StatusCode::SERVICE_UNAVAILABLE
}

pub fn router() -> Router<AppState> {
    Router::new().route("/projects", routing::get(projects))
}
