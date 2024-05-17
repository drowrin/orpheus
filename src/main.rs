use axum::{middleware::from_fn, Router};
use options::apply_options;
use state::{AppState, InitState};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

pub mod options;
pub mod pages;
pub mod state;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(apply_options(
        Router::new()
            .merge(pages::posts::router())
            .merge(pages::home::router())
            .fallback_service(ServeDir::new("./generated/static/"))
            .layer(from_fn(pages::error::handle_error_pages))
            .with_state(AppState::init_state()),
    )
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .into())
}
