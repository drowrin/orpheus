use axum::{extract::Request, middleware::from_fn, Router, ServiceExt};
use options::apply_options;
use state::{AppState, InitState};
use tower::Layer;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer, services::ServeDir,
    trace::TraceLayer,
};

pub mod options;
pub mod pages;
pub mod state;

#[tokio::main]
async fn main() {
    let router = apply_options(
        Router::new()
            .merge(pages::posts::router())
            .merge(pages::home::router())
            .merge(pages::podcasts::router())
            .merge(pages::projects::router())
            .fallback_service(ServeDir::new("./generated/static/"))
            .layer(from_fn(pages::error::handle_error_pages))
            .with_state(AppState::init_state()),
    )
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new());

    let app = NormalizePathLayer::trim_trailing_slash().layer(router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
