use std::time::Duration;

use axum::{
    extract::Request,
    http::{header, HeaderValue},
    middleware::{from_fn, Next},
    response::Response,
    Router,
};
use tokio::time::sleep;
use tower_livereload::LiveReloadLayer;

pub async fn no_cache(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, must-revalidate"),
    );

    response
}

pub async fn simulate_lag(request: Request, next: Next) -> Response {
    sleep(Duration::from_millis(200)).await;

    return next.run(request).await;
}

pub fn apply_options(app: Router) -> Router {
    let mut app = app;

    if let Ok(options) = std::env::var("ORPHEUS_OPTIONS") {
        println!("enabled: {options}");

        if options.contains("live_reload") {
            app = app
                .layer(
                    LiveReloadLayer::new()
                        // don't inject anything into htmx requests
                        .request_predicate(|r: &Request| r.headers().get("HX-Request").is_none())
                        // faster live-reload
                        .reload_interval(Duration::from_millis(100)),
                )
                .into();
        }

        if options.contains("no_cache") {
            app = app.layer(from_fn(no_cache));
        }

        if options.contains("simulate_lag") {
            app = app.layer(from_fn(simulate_lag));
        }
    }

    app
}
