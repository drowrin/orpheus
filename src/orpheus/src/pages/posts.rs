use axum::{
    extract::Path,
    http::StatusCode,
    response::{ErrorResponse, IntoResponse, Response},
    routing, Router,
};
use axum_extra::extract::Query;
use maud::{html, Markup, PreEscaped};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, OnceLock},
};
use tantivy::{collector::TopDocs, doc, schema::Value, TantivyDocument};
use tower_http::services::ServeDir;
use verse::{PostMetaData, SearchMeta, Series};

use super::page::PageKind;

const CHUNK_SIZE: usize = 5;

#[derive(Clone)]
pub struct PostData {
    pub metadata: HashMap<String, PostMetaData>,
    pub series: Vec<Series>,
    pub tags: Vec<String>,
    pub search: Arc<SearchMeta>,
}

static POSTS: OnceLock<PostData> = OnceLock::new();

impl PostData {
    pub fn global() -> &'static Self {
        POSTS.get().expect("PostData is not initialized")
    }

    pub fn init_state() {
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

        if let Err(_) = POSTS.set(PostData {
            metadata,
            series,
            tags,
            search: Arc::new(SearchMeta::open().unwrap()),
        }) {
            println!("WARNING: PostData initialized multiple times");
        }
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
            style="color: var(--color-muted); margin-bottom: 0.5rem;"
            {
                small
                    data-tooltip=[
                        if !post.revisions.is_empty() {
                            Some(post.revisions.clone())
                        } else {
                            None
                        }
                    ]
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
                    hx-swap="innerHTML"
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
) -> Result<impl IntoResponse, ErrorResponse> {
    let post = PostData::global()
        .metadata
        .get(&slug)
        .ok_or(StatusCode::NOT_FOUND)?;

    let path = std::path::Path::new("./generated/posts")
        .join(slug.clone())
        .with_extension("html");

    let post_prose = tokio::fs::read_to_string(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let path_toc = std::path::Path::new("./generated/posts")
        .join(format!("{}-toc", slug.clone()))
        .with_extension("html");

    let post_toc = tokio::fs::read_to_string(&path_toc)
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
                (post_info(&post, html!{ h1 #title {(post.title)} }))
                hr;
                @if post_toc.len() > 40 {
                    details #toc-details {
                        summary { "Table of Contents" }
                        (PreEscaped(post_toc))
                    }
                    hr #toc-hr;
                }
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

pub async fn posts(page_type: PageKind, Query(query): Query<PostsFilters>) -> Response {
    let mut filtered_posts: Vec<&PostMetaData> = if let Some(ref search) = query.search {
        let searcher = PostData::global().search.reader.searcher();
        if let Ok(query) = PostData::global().search.parser.parse_query(search) {
            let results = searcher.search(&query, &TopDocs::with_limit(999)).unwrap();

            results
                .into_iter()
                .flat_map(|(_, addr)| {
                    let doc: TantivyDocument = searcher.doc(addr).unwrap();
                    let slug = doc
                        .get_first(PostData::global().search.fields.slug)
                        .unwrap()
                        .as_str()
                        .unwrap();
                    Some(PostData::global().metadata.get(slug).unwrap())
                })
                .collect()
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }
    } else {
        PostData::global().metadata.values().collect()
    };

    filtered_posts.retain(|m| {
        if let Some(series_slug) = &query.series {
            match &m.series {
                Some(series) if series.slug.eq(series_slug) => (),
                _ => return false,
            }
        }
        query.tag.iter().all(|t| m.tags.contains(t))
    });

    let skip = query.skip.unwrap_or(0);
    let new_query = PostsFilters {
        skip: Some(skip + CHUNK_SIZE),
        ..query.clone()
    };

    if query.search.is_none() {
        filtered_posts.sort_by(|a, b| b.published.cmp(&a.published));
    }
    filtered_posts.drain(0..skip);
    let more_after = filtered_posts.len() > CHUNK_SIZE;
    filtered_posts.truncate(CHUNK_SIZE);

    let posts_markup = html! {
        @if !filtered_posts.is_empty() {
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
                h1 { "Browse Posts" }
                hr;
                form
                    hx-get="/posts"
                    hx-trigger="input changed delay:300ms from:#search, search, change"
                    hx-target="#posts"
                    hx-swap="innerHTML"
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
                                #series-select
                                data-selected={
                                    @if let Some(series) = query.series.clone() {
                                        (series)
                                    }
                                }
                                onchange="this.dataset.selected = this.value"
                                name="series"
                                {
                                    option
                                        style="color: var(--color-muted)"
                                        value=""
                                        selected[query.series.is_none()]
                                        { "Select Series" }
                                    @for series in PostData::global().series.clone() {
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
                                @for tag in PostData::global().tags.clone() {
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
                hr style="margin-top: 0";
                div #posts { (posts_markup) }
            }
        }).into_response()
}

pub fn router() -> Router {
    Router::new()
        .route("/posts", routing::get(posts))
        .route("/posts/:post", routing::get(post))
        .nest_service("/img/", ServeDir::new("./generated/img/"))
}
