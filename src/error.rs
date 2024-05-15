use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use maud::html;

use crate::page::PageKind;

pub fn error_page<T: AsRef<str>>(page_type: PageKind, status: StatusCode, message: T) -> Response {
    let markup = page_type.builder(&message).build(html! {
        span { (message.as_ref()) }
        br;
        a href="javascript:window.history.back();" { "go back" }
    });

    if let PageKind::Boosted = page_type {
        // this is an htmx request, don't send an error code
        markup.into_response()
    } else {
        (status, markup).into_response()
    }
}

pub async fn handle_error_pages(page_type: PageKind, request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    if response.status() == StatusCode::NOT_FOUND {
        return error_page(page_type, StatusCode::NOT_FOUND, "Not Found");
    }

    if response.status() == StatusCode::BAD_REQUEST {
        return error_page(page_type, StatusCode::BAD_REQUEST, "Bad Request");
    }

    if response.status().is_client_error() {
        return error_page(page_type, response.status(), "Client Error");
    }

    if response.status().is_server_error() {
        return error_page(page_type, response.status(), "Internal Server Error");
    }

    response
}
