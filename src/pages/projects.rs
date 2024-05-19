use axum::{response::IntoResponse, routing, Router};
use maud::{html, PreEscaped};

use crate::state::AppState;

use super::page::PageKind;

pub async fn projects(page_kind: PageKind) -> impl IntoResponse {
    page_kind.builder("Home").build(html! {
        div .padded-when-small {
            (PreEscaped(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/pages/projects.html"))))
        }
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/projects", routing::get(projects))
}
