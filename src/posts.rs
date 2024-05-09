use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::{ErrorResponse, IntoResponse, Redirect},
    routing, Router,
};
use axum_extra::extract::Query;
use lyre::{MetaData, Series};
use maud::{html, PreEscaped};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

use crate::{
    page::{column, PageType},
    state::{AppState, InitState},
};

#[derive(Clone)]
pub struct PostData {
    metadata: HashMap<String, MetaData>,
    series: Vec<Series>,
    tags: Vec<String>,
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
                let md = MetaData::open(&path).unwrap();

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

pub async fn post(
    page_type: PageType,
    Path(slug): Path<String>,
    State(posts): State<Posts>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let metadata = posts.metadata.get(&slug).ok_or(StatusCode::NOT_FOUND)?;

    let path = std::path::Path::new("./generated/posts")
        .join(slug.clone())
        .with_extension("html");

    let post_prose = tokio::fs::read_to_string(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(page_type.wrap(
        &metadata.title,
        column(html! {
            header ."mt-6 leading-none space-y-1.5" {
                h1 ."text-4xl m-0" {
                    (metadata.title)
                }
                @if let Some(series) = &metadata.series {
                    p ."text-xl m-0" { a href={"/posts?series="(series.slug)} { (series.name) } }
                }
                @if let Some(tagline) = &metadata.tagline {
                    p ."m-0 italic" { (tagline) }
                }
                div ."flex flex-wrap gap-2" {
                    @for tag in &metadata.tags {
                        a href={ "/posts?tag=" (tag) } { "#"(tag) }
                    }
                }
            }
            hr ."mt-4 mb-2";
            div ."mb-96" {
                (PreEscaped(post_prose))
            }
        }),
    ))
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PostsFilters {
    #[serde(default)]
    tag: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
}

pub async fn posts(
    page_type: PageType,
    Query(query): Query<PostsFilters>,
    State(posts): State<Posts>,
) -> impl IntoResponse {
    let mut filtered_posts: Vec<&MetaData> = posts
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

    filtered_posts.sort_by_key(|m| &m.title);

    let posts_markup = html! {
        div #posts ."flex flex-col" {
            @for post in filtered_posts {
                div ."w-full border border-solid border-slate-400/25 shadow my-1.5 p-2.5" {
                    a
                        ."text-xl"
                        href={ "/posts/" (post.slug) }
                        preload="mouseover"
                        preload-images="true"
                        { (post.title) }
                }
            }
        }
    };

    match page_type {
        PageType::Direct => posts_markup,
        _ => page_type.wrap(
            "Browse Posts",
            column(html! {
                header ."mt-6" {
                    h1 ."text-4xl" { "Browse Posts" }
                }
                form
                    hx-get="/posts"
                    hx-trigger="input changed delay:100ms from:#search, search, change"
                    hx-target="main"
                    hx-push-url="true"
                    "hx-on::config-request"="event.detail.parameters = remove_empty(event.detail.parameters)"
                    {
                        hr ."mt-2 mb-3";
                        div "flex not-prose" {
                            input
                                #search
                                .{
                                    ("w-1/2 mr-[-1px] text-slate-800 border-slate-300 bg-slate-50 ")
                                    ("focus:outline-none focus:ring-0 focus:border-slate-300 focus:bg-white ")
                                }
                                "type"="search"
                                name="search"
                                value=[&query.search]
                                placeholder="Search..."
                            ;
                            select
                                .{
                                    "w-1/2 border-slate-300 bg-slate-50 "
                                    "focus:outline-none focus:ring-0 focus:border-slate-300 focus:bg-white "
                                    @if query.series.is_some() { ("text-slate-800") }
                                    @if query.series.is_none() { ("text-slate-500") }
                                }
                                name="series"
                                {
                                    option
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
                        hr ."mt-3 mb-2";
                        div ."flex flex-wrap gap-2 leading-none" {
                            @for tag in posts.tags.clone() {
                                div {
                                    @let id = format!("checkbox-{tag}");
                                    input
                                        #(id)
                                        "type"="checkbox"
                                        .{"hidden peer"}
                                        checked[query.tag.contains(&tag)]
                                        name="tag"
                                        value=(tag)
                                    ;
                                    label ."peer-checked:font-bold" "for"=(id) {"#" (tag)}
                                }
                            }
                        }
                        hr ."mt-3 mb-2";
                    }
                main { (posts_markup) }
            }),
        )
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/posts/",
            routing::get(|| async { Redirect::permanent("/posts") }),
        )
        .route("/posts", routing::get(posts))
        .route("/posts/:post", routing::get(post))
        .nest_service("/img/", ServeDir::new("./content/img/"))
}
