use axum::{response::IntoResponse, routing, Router};
use maud::html;

use crate::{
    page::{column, PageType},
    state::AppState,
};

pub async fn home_page(page_type: PageType) -> impl IntoResponse {
    page_type.wrap(
        "Home",
        column(html! {
            "Home"
        }),
    )
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(home_page))
}
