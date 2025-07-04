use axum::{extract::Request, middleware::from_fn, Router, ServiceExt};
use tower::Layer;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer, services::ServeDir,
    trace::TraceLayer,
};

pub mod options;
pub mod pages;

#[tokio::main]
async fn main() {
    pages::posts::PostData::init_state();

    let router = options::apply_options(
        Router::new()
            .merge(pages::posts::router())
            .merge(pages::home::router())
            .merge(pages::projects::router())
            .fallback_service(ServeDir::new("./generated/static/"))
            .layer(from_fn(pages::error::handle_error_pages)),
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
