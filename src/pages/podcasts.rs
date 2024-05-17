use axum::{http::StatusCode, response::IntoResponse, routing, Router};

use crate::state::AppState;

pub async fn podcasts() -> impl IntoResponse {
    StatusCode::SERVICE_UNAVAILABLE
}

pub fn router() -> Router<AppState> {
    Router::new().route("/podcasts", routing::get(podcasts))
}
