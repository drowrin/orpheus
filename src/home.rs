use axum::{response::IntoResponse, routing, Router};
use maud::html;

use crate::{page::PageKind, state::AppState};

pub async fn home_page(page_type: PageKind) -> impl IntoResponse {
    page_type.wrap(
        "Home",
        html! {
            "Home"
        },
    )
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(home_page))
}
