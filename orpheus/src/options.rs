#![allow(unused)]

use std::{collections::HashMap, fs, io::BufRead, path::Path, sync::Arc, time::Duration};

use axum::{
    extract::{FromRef, Request, State},
    http::{
        header::{CACHE_CONTROL, ETAG, IF_NONE_MATCH},
        HeaderValue, StatusCode,
    },
    middleware::{from_fn, from_fn_with_state, Next},
    response::{IntoResponse, Response},
    Router,
};
use tokio::time::sleep;
use tower_livereload::LiveReloadLayer;

use crate::AppState;

pub async fn no_cache(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        CACHE_CONTROL,
        HeaderValue::from_static("no-store, must-revalidate"),
    );

    response
}

pub async fn simulate_lag(request: Request, next: Next) -> Response {
    sleep(Duration::from_millis(200)).await;

    next.run(request).await
}

pub fn apply_options(app: Router, state: AppState) -> Router {
    let mut app = app;

    if let Ok(options) = std::env::var("ORPHEUS_OPTIONS") {
        println!("enabled: {options}");

        if options.contains("live_reload") {
            app = app.layer(
                LiveReloadLayer::new()
                    // don't inject anything into htmx requests
                    .request_predicate(|r: &Request| r.headers().get("HX-Request").is_none())
                    // faster live-reload
                    .reload_interval(Duration::from_millis(100)),
            );
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
