use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::{ErrorResponse, IntoResponse},
    routing, Router,
};
use axum_extra::extract::Query;
use maud::{html, Markup, PreEscaped};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use verse::{PostMetaData, Series};

use crate::state::{AppState, InitState};

use super::page::PageKind;

const CHUNK_SIZE: usize = 5;

#[derive(Clone)]
pub struct PostData {
    pub metadata: HashMap<String, PostMetaData>,
    pub series: Vec<Series>,
    pub tags: Vec<String>,
}

pub type Posts = Arc<PostData>;

impl FromRef<AppState> for Posts {
    fn from_ref(input: &AppState) -> Self {
        input.posts.clone()
    }
}

impl InitState for Posts {
    fn init_state() -> Self {
        let mut metadata = HashMap::new();

        for path in std::fs::read_dir("./generated/posts").unwrap() {
            let path = path.unwrap().path();
            if matches!(path.extension(), Some(ext) if ext == "yml") {
                let md = PostMetaData::open(&path).unwrap();

                metadata.insert(md.slug.clone(), md);
            }
        }

        let collect_tags: HashSet<String> =
            metadata.values().flat_map(|m| m.tags.clone()).collect();
        let mut tags: Vec<String> = collect_tags.into_iter().collect();

        tags.sort();

        let collect_series: HashSet<Series> =
            metadata.values().flat_map(|m| m.series.clone()).collect();
        let mut series: Vec<Series> = collect_series.into_iter().collect();

        series.sort_by_key(|s| s.slug.to_owned());

        Arc::new(PostData {
            metadata,
            series,
            tags,
        })
    }
}

pub fn post_info(post: &PostMetaData, title: Markup) -> Markup {
    html! {
        hgroup
            style="margin: 0;"
            {
                (title)
                @if let Some(tagline) = &post.tagline {
                    p
                        { (tagline) }
                }
            }
        div style="display: flex; flex-wrap: wrap;" {
            @if let Some(series) = &post.series {
                a .tag href={"/posts?series="(series.slug)} { (series.name) }
            }
            div style="display: flex; flex-wrap: wrap;" {
                    @for tag in &post.tags {
                    a .tag href={ "/posts?tag=" (tag) } { "#"(tag) }
                }
            }
        }
        div
            style="color: var(--pico-muted-color); margin-bottom: 0.5rem;"
            {
                small
                    data-tooltip=[post.updated.as_ref().map(|u| format!("updated {}", u))]
                    data-placement="right"
                    { (post.published) }
                " - "
                small
                    data-tooltip={ (post.word_count) " words" }
                    data-placement="right"
                    { (post.reading_time) " minutes" }
            }
    }
}

pub fn post_card(post: &PostMetaData) -> Markup {
    html! {
        article {
            (post_info(&post, html! {
                h3 { a
                    ."article-link"
                    href={ "/posts/" (post.slug) }
                    preload="mouseover"
                    preload-images="true"
                    { (post.title) }
                }
            }))

            hr style="margin: 0.5rem 0 0.4rem 0";

            span .truncate { (post.brief) }
        }
    }
}

pub async fn post(
    page_type: PageKind,
    Path(slug): Path<String>,
    State(posts): State<Posts>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let post = posts.metadata.get(&slug).ok_or(StatusCode::NOT_FOUND)?;

    let path = std::path::Path::new("./generated/posts")
        .join(slug.clone())
        .with_extension("html");

    let post_prose = tokio::fs::read_to_string(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(page_type
        .builder(&post.title)
        .with_description(if let Some(tagline) = post.tagline.clone() {
            format!("{tagline}\n---\n{}", post.brief.clone())
        } else {
            post.brief.clone()
        })
        .build(html! {
            article .prose {
                (post_info(&post, html!{ h1 {(post.title)} }))
                hr;
                (PreEscaped(post_prose))
            }
        }))
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PostsFilters {
    #[serde(default)]
    tag: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<usize>,
}

pub async fn posts(
    page_type: PageKind,
    Query(query): Query<PostsFilters>,
    State(posts): State<Posts>,
) -> impl IntoResponse {
    let mut filtered_posts: Vec<&PostMetaData> = posts
        .metadata
        .values()
        .filter(|m| {
            if let Some(series_slug) = &query.series {
                match &m.series {
                    Some(series) if series.slug.eq(series_slug) => (),
                    _ => return false,
                }
            }
            if let Some(search) = &query.search {
                if !m
                    .title
                    .to_lowercase()
                    .contains(search.to_lowercase().as_str())
                {
                    return false;
                }
            }
            query.tag.iter().all(|t| m.tags.contains(t))
        })
        .collect();

    let skip = query.skip.unwrap_or(0);
    let mut new_query = query.clone();
    new_query.skip = Some(skip + CHUNK_SIZE);

    filtered_posts.sort_by(|a, b| b.published.cmp(&a.published));
    filtered_posts.drain(0..skip);
    let more_after = filtered_posts.len() > CHUNK_SIZE;
    filtered_posts.truncate(CHUNK_SIZE);

    let posts_markup = html! {
        @if filtered_posts.len() > 0 {
            @if more_after {
                @for post in &filtered_posts[0..filtered_posts.len()-1] {
                    div { (post_card(post)) }
                }
                @if let Some(post) = filtered_posts.last() {
                    div
                        hx-get={"/posts?" (serde_html_form::to_string(new_query).unwrap())}
                        hx-swap="afterend"
                        hx-trigger="revealed"
                        { (post_card(post)) }
                }
            } @else {
                @for post in filtered_posts {
                    div { (post_card(post)) }
                }
            }
        }
    };

    page_type
        .builder("Browse Posts")
        .on_direct_request(posts_markup.clone())
        .with_description("Browse and filter all blog posts")
        .build(html! {
            div ."padded-when-small" {
                h1 style="margin-bottom: -0.3rem" { "Browse Posts" }
                hr;
                form
                    hx-get="/posts"
                    hx-trigger="input changed delay:100ms from:#search, search, change"
                    hx-target="#posts"
                    hx-swap="innerHTML"
                    hx-push-url="true"
                    "hx-on::config-request"="event.detail.parameters = remove_empty(event.detail.parameters)"
                    {
                        fieldset
                            role="group"
                            style="margin-bottom: 0.5rem;"
                            {
                            input
                                #search
                                "type"="search"
                                name="search"
                                value=[&query.search]
                                placeholder="Search..."
                            ;
                            select
                                #series-select
                                data-selected={
                                    @if let Some(series) = query.series.clone() {
                                        (series)
                                    } @else { "" }
                                }
                                onchange="this.dataset.selected = this.value"
                                name="series"
                                {
                                    option
                                        style="color: var(--pico-form-element-placeholder-color)"
                                        value=""
                                        selected[query.series.is_none()]
                                        { "Select Series" }
                                    @for series in posts.series.clone() {
                                        option
                                            value=(series.slug)
                                            selected[matches!(
                                                query.series,
                                                Some(ref s) if s.clone() == series.slug
                                            )]
                                            { (series.name) }
                                    }
                                }
                            }
                        fieldset
                            style="margin-bottom: -0.15rem;"
                            {
                                @for tag in posts.tags.clone() {
                                    @let id = format!("checkbox-{tag}");
                                    input
                                        #(id)
                                        "type"="checkbox"
                                        checked[query.tag.contains(&tag)]
                                        name="tag"
                                        value=(tag)
                                    ;
                                    label
                                        .tag
                                        "for"=(id)
                                        {"#" (tag)}
                                }
                            }
                    }
                hr;
                div #posts { (posts_markup) }
            }
        })
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/posts", routing::get(posts))
        .route("/posts/:post", routing::get(post))
        .nest_service("/img/", ServeDir::new("./generated/img/"))
}
