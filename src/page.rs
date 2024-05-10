use std::convert::Infallible;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use maud::{html, Markup, DOCTYPE};

pub fn column(markup: Markup) -> Markup {
    html! {
        div ."mx-3 flex items-center justify-center" {
            div ."w-full prose prose-slate dark:prose-invert" {
                (markup)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum PageKind {
    Direct,
    Boosted,
    Full,
}

#[async_trait]
impl<S> FromRequestParts<S> for PageKind
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if matches!(parts.headers.get("HX-Boosted"), Some(v) if v == "true") {
            return Ok(Self::Boosted);
        }

        if matches!(parts.headers.get("HX-Request"), Some(v) if v == "true") {
            return Ok(Self::Direct);
        }

        return Ok(Self::Full);
    }
}

impl PageKind {
    pub fn wrap<S: AsRef<str>>(self, title: S, content: Markup) -> Page {
        Page::new(self, title, content)
    }
}

pub struct Page {
    kind: PageKind,
    title: String,
    content: Markup,
    head: Option<Markup>,
    direct: Option<Markup>,
}

impl Page {
    pub fn new<S: AsRef<str>>(kind: PageKind, title: S, content: Markup) -> Self {
        Self {
            kind: kind,
            title: title.as_ref().into(),
            content: content,
            head: None,
            direct: None,
        }
    }

    pub fn with_head(mut self, head: Markup) -> Self {
        self.head = match self.head {
            Some(current) => Some(html! {
                (current)
                (head)
            }),
            None => Some(head),
        };
        self
    }

    pub fn with_description<S: AsRef<str>>(self, description: S) -> Self {
        self.with_head(html! {
            meta name="description" content=(description.as_ref());
        })
    }

    pub fn on_direct_request(mut self, direct: Markup) -> Self {
        self.direct = Some(direct);
        self
    }
}

impl From<Page> for Markup {
    fn from(page: Page) -> Self {
        let navbar_separator = html! {
            span ."text-sm text-slate-400 dark:text-slate-700" { "|" }
        };
        let has_head = page.head.is_some();
        let head = html! {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" href="/styles.css";
                link rel="icon" href="/favicon.ico" sizes="any";
                script src="/common.js" {}
                script src="https://unpkg.com/htmx.org@1.9.12/dist/htmx.min.js" {}
                script src="https://unpkg.com/htmx.org@1.9.12/dist/ext/preload.js" {}
                script src="https://unpkg.com/htmx.org@1.9.12/dist/ext/head-support.js" {}
                title { (page.title) }
                @if let Some(append_head) = page.head {
                    (append_head)
                }
            }
        };
        let navbar = html! {
            div
                .{
                    "px-3 py-1.5 flex justify-center fixed "
                    "top-0 left-0 w-full bg-slate-200 dark:bg-slate-900"
                }
                {
                    div
                        ."flex w-[65ch] text-slate-600 dark:text-slate-400 leading-none"
                        {
                            nav
                                ."space-x-2"
                                {
                                    a href="/" { "Home" }
                                    (navbar_separator)
                                    a href="/posts" { "Posts" }
                                }
                            svg
                                #toggle-dark-mode
                                ."ml-auto text-slate-500 dark:text-slate-500"
                                xmlns="http://www.w3.org/2000/svg"
                                onclick="toggle_dark_mode()"
                                title="Toggle Theme"
                                width="20px"
                                height="20px"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke-width="1.5"
                                stroke="currentColor"
                                {
                                    path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    ;
                                }
                        }
                    div
                        #loading-bar
                        .{
                            "absolute -bottom-0.5 left-0 "
                            "w-full h-0.5 "
                            "bg-slate-400 dark:bg-slate-600 "
                            "opacity-0 "
                        }
                        {}
                }
        };
        match page.kind {
            PageKind::Direct => match page.direct {
                Some(direct) => direct,
                None => page.content,
            },
            PageKind::Boosted => html! {
                @if has_head {
                    (head)
                } else {
                    title { (page.title) }
                }
                (navbar)
                (page.content)
            },
            PageKind::Full => html! {
                (DOCTYPE)
                (head)
                html lang="en" {
                    body
                        hx-boost="true"
                        hx-ext="preload,head-support"
                        hx-indicator="#loading-bar"
                        ."bg-slate-100 dark:bg-slate-950 pt-8"
                        {
                            (navbar)
                            (page.content)
                        }
                }
            },
        }
    }
}

#[async_trait]
impl IntoResponse for Page {
    fn into_response(self) -> Response {
        Markup::from(self).into_response()
    }
}
