use axum::{middleware::from_fn, Router};
use error::handle_error_pages;
use options::apply_options;
use state::{AppState, InitState};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

mod error;
mod home;
mod options;
mod page;
mod posts;
mod state;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(apply_options(
        Router::new()
            .merge(posts::router())
            .merge(home::router())
            .fallback_service(ServeDir::new("./generated/static/"))
            .layer(from_fn(handle_error_pages))
            .with_state(AppState::init_state()),
    )
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .into())
}
