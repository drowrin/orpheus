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
