use axum::{extract::State, response::IntoResponse, routing, Router};
use maud::html;

use crate::{pages::posts::post_card, AppState, Posts};

use super::page::PageKind;

pub async fn home_page(page_type: PageKind, State(posts): State<Posts>) -> impl IntoResponse {
    let mut posts = posts.metadata.values().collect::<Vec<_>>();
    posts.sort_by(|a, b| b.published.cmp(&a.published));

    page_type.builder("Home").build(html! {
        div .padded-when-small {
            hgroup {
                h1 #drowrin {
                    img
                        src="/favicon.svg"
                        title="logo"
                        style="max-height: 0.75em";
                    "Drowrin"
                }
                p {
                    "Software Engineering;" br;
                    "Media Reviews;" br;
                    "TTRPGs;" br;
                }
            }
            section {
                hgroup {
                    h2 { "Recent Posts" }
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
