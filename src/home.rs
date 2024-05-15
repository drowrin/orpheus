use axum::{http::StatusCode, response::IntoResponse, routing, Router};
use maud::html;

use crate::{page::PageKind, state::AppState};

pub async fn home_page(page_type: PageKind) -> impl IntoResponse {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        page_type.builder("Home").build(html! {
            "Home"
        }),
    )
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(home_page))
}
