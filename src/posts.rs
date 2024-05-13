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
    page::PageKind,
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
    page_type: PageKind,
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
        html! {
            hgroup
                {
                    h1
                        {
                            (metadata.title)
                        }
                    @if let Some(tagline) = &metadata.tagline {
                        p
                            { (tagline) }
                    }
                }
            @for tag in &metadata.tags {
                a ."tag" href={ "/posts?tag=" (tag) } { "#"(tag) }
            }
            @if let Some(series) = &metadata.series {
                a href={"/posts?series="(series.slug)} { (series.name) }
            }
            hr;
            (PreEscaped(post_prose))
        },
    ).with_description(metadata.brief.clone()))
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
    page_type: PageKind,
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
        div
            #posts
            {
                @for post in filtered_posts {
                    article
                        {
                            a
                                href={ "/posts/" (post.slug) }
                                preload="mouseover"
                                preload-images="true"
                                { (post.title) }
                            br;
                            (post.brief)
                        }
                }  
            }
    };

    page_type.wrap(
        "Browse Posts",
        html! {
            h1 { "Browse Posts" }
            hr;
            form
                hx-get="/posts"
                hx-trigger="input changed delay:100ms from:#search, search, change"
                hx-target="#posts"
                hx-push-url="true"
                "hx-on::config-request"="event.detail.parameters = remove_empty(event.detail.parameters)"
                {
                    fieldset
                        role="group"
                        {
                        input
                            #search
                            "type"="search"
                            name="search"
                            value=[&query.search]
                            placeholder="Search..."
                        ;
                        select
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
                    fieldset
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
            (posts_markup)
        },
    )
    .on_direct_request(posts_markup)
    .with_description("Browse and filter all blog posts")
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
