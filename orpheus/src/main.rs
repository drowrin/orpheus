use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{
    extract::{FromRef, Request},
    middleware::from_fn,
    Router, ServiceExt,
};
use options::apply_options;
use tower::Layer;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer, services::ServeDir,
    trace::TraceLayer,
};
use verse::{PostMetaData, SearchMeta, Series};

pub mod options;
pub mod pages;

#[derive(Clone)]
pub struct PostData {
    pub metadata: HashMap<String, PostMetaData>,
    pub series: Vec<Series>,
    pub tags: Vec<String>,
    pub search: Arc<SearchMeta>,
}

pub type Posts = Arc<PostData>;

impl FromRef<AppState> for Posts {
    fn from_ref(input: &AppState) -> Self {
        input.posts.clone()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub posts: Posts,
}

impl AppState {
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

        AppState {
            posts: Arc::new(PostData {
                metadata,
                series,
                tags,
                search: Arc::new(SearchMeta::open().unwrap()),
            }),
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::init_state();
    let router = apply_options(
        Router::new()
            .merge(pages::posts::router())
            .merge(pages::home::router())
            .merge(pages::projects::router())
            .fallback_service(ServeDir::new("./generated/static/"))
            .layer(from_fn(pages::error::handle_error_pages))
            .with_state(state.clone()),
        state,
    )
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new());

    let app = NormalizePathLayer::trim_trailing_slash().layer(router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Starting server...");

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
