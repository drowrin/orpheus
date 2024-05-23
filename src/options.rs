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

use crate::state::{AppState, InitState};

pub type ETags = Arc<HashMap<String, String>>;

impl InitState for ETags {
    fn init_state() -> Self {
        Arc::new(HashMap::from_iter(
            fs::read("generated/etags").unwrap().lines().map(|line| {
                let line = line.unwrap();
                let mut i = line.split(":");
                (i.next().unwrap().to_string(), i.next().unwrap().to_string())
            }),
        ))
    }
}

impl FromRef<AppState> for ETags {
    fn from_ref(input: &AppState) -> Self {
        input.etags.clone()
    }
}

pub async fn cache(State(etags): State<ETags>, request: Request, next: Next) -> Response {
    let path = Path::new(&request.uri().to_string())
        .with_extension("")
        .to_str()
        .unwrap()
        .to_string();
    let maybe_etag = etags.get(&path);

    if let Some(browser_etag) = request.headers().get(IF_NONE_MATCH) {
        if let Some(etag) = maybe_etag {
            if browser_etag.to_str().unwrap() == etag.as_str() {
                return StatusCode::NOT_MODIFIED.into_response();
            }
        }
    }

    let mut response = next.run(request).await;

    response.headers_mut().insert(
        CACHE_CONTROL,
        HeaderValue::from_static("private, max-age=3600"),
    );

    if let Some(etag) = maybe_etag {
        response
            .headers_mut()
            .insert(ETAG, HeaderValue::from_str(etag).unwrap());
    }

    response
}

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

    return next.run(request).await;
}

pub fn apply_options(app: Router, state: AppState) -> Router {
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
        } else {
            app = app.layer(from_fn_with_state(state, cache));
        }

        if options.contains("simulate_lag") {
            app = app.layer(from_fn(simulate_lag));
        }
    }

    app
}
