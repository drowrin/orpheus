use axum::{extract::State, response::IntoResponse, routing, Router};
use maud::{html, PreEscaped};
use tokio::fs;

use crate::{pages::posts::post_card, state::AppState};

use super::{page::PageKind, posts::Posts};

pub async fn home_page(page_type: PageKind, State(posts): State<Posts>) -> impl IntoResponse {
    let mut posts = posts.metadata.values().collect::<Vec<_>>();
    posts.sort_by(|a, b| b.published.cmp(&a.published));

    let markup = fs::read_to_string("generated/pages/home.html")
        .await
        .unwrap();

    page_type.builder("Home").build(html! {
        div .padded-when-small {
            (PreEscaped(markup))
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
