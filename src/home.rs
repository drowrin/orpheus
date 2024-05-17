use axum::{extract::State, response::IntoResponse, routing, Router};
use maud::{html, PreEscaped};

use crate::{
    page::PageKind,
    posts::{post_card, Posts},
    state::AppState,
};

pub async fn home_page(page_type: PageKind, State(posts): State<Posts>) -> impl IntoResponse {
    let mut posts = posts.metadata.values().collect::<Vec<_>>();
    posts.sort_by(|a, b| b.published.cmp(&a.published));

    page_type.builder("Home").build(html! {
        div .padded-when-small {
            (PreEscaped(include_str!("../generated/pages/home.html")))
            section {
                hgroup {
                    h2 { "Recent Posts" }
                    p {
                        "check out my "
                        a href="/posts" { "blog" }
                    }
                }
                @for post in posts.into_iter().take(3) {
                    (post_card(post))
                }
            }
        }
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(home_page))
}
