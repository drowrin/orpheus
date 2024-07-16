use std::convert::Infallible;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use maud::{html, Markup, DOCTYPE};

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
    pub fn builder<S: AsRef<str>>(self, title: S) -> PageBuilder {
        PageBuilder::new(self, title)
    }
}

pub struct PageBuilder {
    kind: PageKind,
    title: String,
    head: Option<Markup>,
    direct: Option<Markup>,
}

pub struct Page {
    content: Markup,
    kind: PageKind,
    title: String,
    head: Option<Markup>,
    direct: Option<Markup>,
}

impl PageBuilder {
    pub fn new<S: AsRef<str>>(kind: PageKind, title: S) -> Self {
        Self {
            kind,
            title: title.as_ref().into(),
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

    pub fn build(self, content: Markup) -> Page {
        Page {
            content,
            kind: self.kind,
            title: self.title,
            head: self.head,
            direct: self.direct,
        }
    }
}

impl From<Page> for Markup {
    fn from(page: Page) -> Self {
        let has_head = page.head.is_some();

        let head = html! {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link
                    rel="stylesheet"
                    href="/styles.css";
                link
                    rel="icon"
                    href="/favicon.svg"
                    sizes="any";
                script
                    src="/main.js"
                    {}
                script
                    src="/htmx.js"
                    {}
                script
                    src="/preload.js"
                    {}
                script
                    src="/head-support.js"
                    {}
                title { "drowrin.com | " (page.title) }
                @if let Some(append_head) = page.head {
                    (append_head)
                }
            }
        };

        let navbar = html! {
            nav ."container padded-when-small" {
                    ul {
                        li { a href="/" { "Home" } }
                        li { a href="/posts" { "Posts" } }
                        li { a href="/podcasts" { "Podcasts" } }
                        li { a href="/projects" { "Projects" } }
                    }
                    ul {
                        li
                            data-tooltip="Toggle Theme"
                            data-placement="left"
                            style="display: flex; align-items: center"
                            onclick="toggle_dark_mode()"
                            {
                                svg
                                    #toggle-dark-mode
                                    xmlns="http://www.w3.org/2000/svg"
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
                    }

            }
            div
                style="position: fixed; top: 1.75rem; left: 0; width: 100%; max-width: 100%; pointer-events: none"
                {
                    div
                        style="width: 100%; max-width: var(--readable-width); margin: auto"
                        { progress #loading-bar; }
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
                } @else {
                    title { (page.title) }
                }
                header {
                    (navbar)
                }
                main .container {
                    (page.content)
                }
            },
            PageKind::Full => html! {
                (DOCTYPE)
                (head)
                html lang="en" {
                    body
                        hx-boost="true"
                        hx-ext="preload,head-support"
                        hx-indicator="#loading-bar"
                        {
                            header {
                                (navbar)
                            }
                            main .container {
                                (page.content)
                            }
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
