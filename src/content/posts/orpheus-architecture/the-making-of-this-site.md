---
title: The Making of this Site
series: Orpheus Architecture
tagline: Everything Before the Site Went Live
tags:
  - programming
  - projects
---

I've wanted a blog for a while. I've wanted a personal website at the root of
this domain for a while. I've wanted to explore a handle of web technologies
for a while. So, it was time to get tinkering.

I initially tried a few different site generators. I knew I wanted to write
primarily in markdown. I also knew I wanted the site to be fairly minimal. There
are some good site generators out there, but none fit quite what I wanted. I even
tried one that would convert notes I wrote in [Obsidian](https://obsidian.md/)
and publish them to a site. I decided it would be more fun to just make something
myself. Not that I think this will be better than anything out there, it'll just
be more personal. Plus, _it's fun to tinker_.

I'll organize this post into logical sections, and within each I'll explain the
thought process and exploration that got me to the point I thought the site was
good enough to publish. I'll also give an overview of the current architecture.

This project is split into two major parts:

- Orpheus, the website itself
- Lyre, the preprocessor and build tool

As I update the site, I will add more posts to this
[series](/posts?series=orpheus-architecture) and leave this post as is. That
way, the history of my development process will be preserved. Though I may come
back to this post to add links and updates when things get wildly out of date.

---

# Orpheus

I designed Orpheus to be as lean as possible. I wanted it to serve pages as
efficiently as possible, without requiring much live server-side processing. I
also wanted to keep the binary size small and avoid relying on external tools at
runtime. So as much work as possible is offloaded to the Lyre preprocessor.
Also, minimal dependencies mean fast build times, which is great for being able
to live-reload content and quickly deploy my changes. A live-reload takes about
100ms on my machine. A deployment to my production server takes about 5 seconds.

---

## Backend Framework

Admittedly, I already knew what I wanted for this:
[axum](https://github.com/tokio-rs/axum). I used axum when I participated in the
[Christmas Code Hunt](https://www.shuttle.rs/cch) by
[shuttle.rs](https://www.shuttle.rs/) and I loved it. So there wasn't much
experimentation here. However, I _did_ get to explore deeper into axum's features.
I learned a lot, and still feel like I've only scratched the surface of **how
powerful axum can be**.

I find Axum very ergonomic and flexible. Here's an example:

```rs
async fn posts(
    Query(query): Query<PostsFilters>,
    State(posts): State<Posts>,
) -> impl IntoResponse {
  ...
}

pub async fn post(
    Path(slug): Path<String>,
    State(posts): State<Posts>,
) -> Result<impl IntoResponse, ErrorResponse> {
  ...
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/posts", routing::get(posts))
        .route("/posts/:post", routing::get(post))
}
```

This creates a `Router` that calls the function `posts` whenever there is a
`GET` request on the endpoint `/posts`, and the function `post` whenever there
is a `GET` request on `/posts/<post-slug>`.

### Extracting Context

Axum automatically passes all required arguments to the function, so long as an
Extractor is defined for them. So if I want access to my application state, all
I need to do is add that state as an argument to the function. If I want
specific sections of the requested URL, I can add them as other arguments.
Everything just works. I don't need to add more parameters than I need. I don't
need access to any large, overkill context structs. I could even have a function
with no parameters if I want. It's minimal in a powerful way, with easy access
to expand to anything I need.

You may also have noticed that there is a `Router<AppState>` type in the above
example, but I access a `State<Posts>` type in the functions. This is because
axum allows arbitrary types to implement a `FromRef` trait, which can do
transformations like this. I use it to access only the portions of state I need.
I've defined my state type for all my `posts` endpoints in the same file as
functions, and implemented `FromRef<AppState>` for it. This way, my `posts`
functions don't need to know what the rest of the application state looks like,
and I avoid needing to write `state.posts` everywhere. **All automatically!** It
leads to great locality of behavior, and a great development experience.

### Responding

Additionally, axum will automatically transform any into an intelligently
crafted response with sane defaults, so long as it implements `IntoResponse`.
Thankfully, axum has blanket implementations of this for a wide variety of
types, including things like tuples of `impl IntoResponse`. So things like this,
once again, _just work_:

```rs
async fn home_page() -> impl IntoResponse {
  (StatusCode::SERVICE_UNAVAILABLE, String::from("under construction"))
}
```

Results also work, so long as both sides of the `Result` implement
`IntoResponse`. This makes error handling in axum extremely easy. I can use
rust's great `?` operator to greatly streamline the code and focus on the core,
happy-path logic. Then, I can create my own error type with `From` implemented
for whatever errors I want to deal with, give it an implementation of
`IntoResponse`, and put all that behavior in one place.

This is great, because it gives me all the advantages of rust's excellent error
handling, while still allowing an easy way to define global handlers for common
errors. This is very convenient and flexible. If I want a global error response
for the case where a file is not found, I can have that. If I want to break out
of that and handle that error differently in a specific function, all I have to
do is remove one `?` and handle the error myself in that function. Or I could
even create a custom error for that case and map to it. The possibilities are
many, and it's trivial to switch to the appropriate method.

### Middleware

Axum uses the `tower` ecosystem for middleware. There's tons available. With
just a few lines of code I was able to add compression, logging, path
normalization, and even live-reloading:

```rs
Router::new()
    .merge(pages::posts::router()) // the router from the above example
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .layer(LiveReloadLayer::new())
```

The builder pattern here makes it trivial to add or remove middleware layers
during the server startup. For example, if I don't want live-reloading on
the release profile:

```rs
let mut app = Router::new()
    .merge(pages::posts::router()) // the router from the above example
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new());

if cfg!(debug_assertions) {
    app = app.layer(LiveReloadLayer::new());
}
```

I can also do things like disable caching in debug mode by writing custom
middleware, which tower makes easy:

```rs
async fn no_cache(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, must-revalidate"),
    );

    response
}

if cfg!(debug_assertions) {
    app = app.layer(from_fn(no_cache));
}
```

---

## Frontend Library

For this site, I didn't have any crazy reactivity requirements. There isn't
really any client state to deal with here. All this website does is request data
from the server and display it. It doesn't need a complicated framework. This
sounds like a perfect job for [htmx](https://htmx.org/)!

After using htmx for a bit, I think it would be very capable of much more
complicated designs. That, however, is a topic for another day. Perhaps I'll use
it again in the future for a more complicated project. The experience has been
**great**!

The way you access (almost) all of htmx's features is HTML attributes. You
generally don't need to break out into javascript, because the attributes
are very powerful. It allows site behavior to be defined declaratively,
local to the elements involved in the behavior, without losing the context
of surrounding elements.

### Instant SPA

```html
<body hx-boost="true">
  ...
</body>
```

The htmx attribute `hx-boost` handles a massive amount of cases instantly. It
hijacks all local links on the site and replaces them with AJAX requests to
fetch the `body` HTML, and live-swaps it into the DOM. This gives an
otherwise-static site that Single Page Application (SPA) feeling.
No more page loads, no need to load `<head>` links again, no loss of state. It
even handles smooth scrolling to the new content. All effortlessly.

On the backend, I watch for the `HX-Boosted` header and change what HTML I send
back. If the header isn't present, I need to send the whole page. If it is, I
can just send the `body`.

I also use the `preload` htmx extension. It causes the AJAX request to be fired
when a user hovers over the link, and then performs the swap on click. On
mobile, this works based on the start of the touch event, instead of the end of
it. This means we get a few hundred milliseconds of head-start on every page
load, which improves the responsiveness of the site a lot.

I've also added a global loading-indicator at the top of the site. This will
fade in whenever htmx is waiting on requests, and fade back out when it is done.
This can be overridden for specific components if needed, but the global fallback
is nice for showing the user that something _is happening_ when they click.

#### But what if I need to change the title?

Fortunately, htmx has a solution for this. If you send back a `<title>` element,
it will intelligently swap that where it needs to go. It can also be configured
to run scripts that are part of the response, and more.

I use the `head-support` htmx plugin, which will be merged in as a core feature
in the next major version of htmx. It allows `<head>` tags that the backend sends
to be intelligently merged with the existing head, removing elements no longer
present in the new `<head>` and only running new elements that were not present
in the previous `<head>`. This is not required for `<title>` swaps to work, but
it's handy for other descriptive meta tags that might vary from page to page.

#### More work on the backend?

Thanks to some axum superpowers, I wrote some helpers to do all this
transformation for me as necessary. I just construct a page response as I
normally would, and my helpers take care of the rest. That way I don't need to
think about this stuff while writing an endpoint. I can just focus on the logic
specific to that endpoint.

I found this solution to be quite elegant and easy to work with.

### Declarative Search Page

Here is a reduced version of the code for the [Browse Posts](/posts) page on
this site. It has styles and labels removed, and only a couple values present,
so that it is a more minimal example. Regardless of everything removed, this
would function just as the full version of the code does--it would just look
different.

```html
<form
  hx-get="/posts"
  hx-trigger="input changed delay:100ms from:#search, search, change"
  hx-target="#posts"
  hx-swap="outerHTML"
  hx-push-url="true"
>
  <fieldset>
    <input id="search" type="search" name="search" placeholder="Search..." />

    <select name="series">
      <option value="ttrpg-design-rants">TTRPG Design Rants</option>
      <option value="annual-media-rankings">Annual Media Rankings</option>
    </select>
  </fieldset>

  <fieldset>
    <input type="checkbox" name="tag" value="opinions" />
    <input type="checkbox" name="tag" value="games" />
  </fieldset>
</form>

<div id="posts"></div>
```

As you can see, it's a pretty normal `<form>` other than the `hx-*` attributes on
the root `<form>` element. I'll break down these attributes one by one to explain
how this works.

#### hx-get

This attribute identifies what URL should be accessed when the form is submitted,
and specifies that the `GET` method should be used.

This attribute can be used on many other elements too. Links are an easy example,
but it could go on any element that can be "activated" in some way.

With no other configuration, htmx would attempt to grab `/posts` and place the
response inside the calling element. In this case, that would be the `<form>`.
We obviously don't want that behavior here, so we use the other attributes to
modify what happens.

#### hx-trigger

This sets how the element activates and sends the htmx request. In this case, I
have allowed any of 3 events to trigger the `<form>`:

- `input changed delay:100ms from:#search`  
  If the input changes in the `#search` element, after a period of 100ms without
  change, trigger the element. This allows us to respond to a user's search as
  they type, without triggering on every key press.
- `search`  
  If the user triggers the search by hitting enter, trigger the element.
- `change`  
  If any child element's value changes, trigger the element.

There are many possibilities with this trigger attribute. These are just the
triggers that I found work best for this particular use case.

#### hx-target

By default, `hx-get` would attempt to swap the returned HTML as the `innerHTML`
of the `<form>` element. The `hx-target` attribute allows us to select a
different element. CSS selectors can be used, but there are also some utility
modifiers provided by htmx. I just use a CSS selector here.

#### hx-swap

By default, `hx-get` would attempt to swap the returned HTML as the `innerHTML`
of the target. This lets us swap the returned HTML somewhere else. Here, I
simply swap the `outerHTML` to replace the `#posts` div entirely, but there are
other possible values too. You could swap content after the existing content,
before the target, or many other places.

#### hx-push-url

This tells htmx to update the browser's URL to the URL it requests. It also adds
an entry to the browser history. For this use case, that means that the URL will
update as the user interacts with the Search UI. They can copy or bookmark the
URL to get an easy way to access the same filters again later, or to share the
filters.

#### The Rest of the Form

Everything else is pretty standard. These form elements would be similar in any
other frontend framework or library. Forms and URL queries are just a great way
to handle this. Nothing about the form structure was required by htmx, it's
just normal form design. You could even strip the htmx attributes from it and
it would function as a form to be used as usual.

The htmx attributes just enhance the form with extra functionality. It only took
5 lines of code to provide this nice functionality, and they're declarative lines
of code to boot!

These attributes are powerful building blocks.

### Overall HTMX Impressions

I _really, really_ like HTMX.

I don't think it's right for _everything_, but it doesn't try to be right for
everything. What it's good for is getting state between the front and backend
that can be represented as HTML. Not everything can be represented as HTML,
but **a lot** can be.

For more directly interactable content where server-client state syncs are rare,
a different approach is better for the frontend. However, even then--I'd love to
use HTMX for all the actual state management, and let the other client-side
strategies handle the client-only state.

I love the declarative nature of htmx and the locality of behavior it provides.
It also just feels semantically like HTML with superpowers, so it's very natural
to write. Far more natural than any javascript-focused frontend framework or
library I've used.

I also love the kind of backend it lets me design. I'm working with composable
HTML templates in a fairly functional style, which is a mental model that just
**works** really well for me personally.

---

## Style

At first, I started with [tailwindcss](https://tailwindcss.com/). I had heard
about it many times, and have really wanted to give it a shot. I generally found
it enjoyable. However, as I used it, I found the utility classes really started
piling up. It made other parts of my code hard to read. The style was starting
to dominate the functionality.

I could split stuff into custom classes, and other such strategies to reduce the
large number of classes in my code. However, this just started to feel like
regular CSS again. The benefits of tailwind started to diminish. I could do the
same by using inline styles while prototyping and then moving things to classes
when I wanted share behavior.

I'm sure if I was a real designer and had a vision in mind ahead of time,
tailwind would be great. It allows for a ton of flexibility and precision. I
just... don't really need all that on this site. This is a minimal site.
Tailwind was overkill.

### PicoCSS

So I switched to [Pico CSS](https://picocss.com/) and was immediately impressed.

It provides good defaults for semantic HTML, which I strive to write anyways. So
I get a lot of benefit, essentially for free. I had to change my idea of what
semantic HTML looks like in a few cases, but that's acceptable.

This gives me an excellent base to work with, and then I can define my own
classes on top of it if I want.

It's also super easy to hack. I'm still learning the best way to accomplish what
I want (I think I could reduce my output CSS file by a lot with some more work),
but I was able to get my preferred theme, layout tweaks, and more--all with very
simple CSS variables.

Honestly, I don't have a ton to say about it, because it's so simple and it just
works. For a simple site like this, it's **perfect**.

---

## HTML Rendering

At first, I used [Askama](https://github.com/djc/askama). I think it's a super
cool project. You get type safety and compile-time checks on HTML template
files. It uses a clever macro to load the templates and compare them to a struct
you define for each template, to ensure that all inputs are accounted for and
expected. You can then create that struct using whatever method you prefer and
render it using whatever method you prefer. It's that easy.

Unfortunately, I found it a little cumbersome for this project. It might just be
a little overkill for such a simple site.

I'm working with HTMX, so rendering small fragments that are easily composed
into larger templates needs first-class support. I can kind of get this with
Askama, but it requires more steps. Askama is engineered more towards
inheritance in its templates, [Jinja](https://jinja.palletsprojects.com/) style
(which makes sense, as Askama is super Jinja-inspired). Not only do I find this
unergonomic, I just don't like the inheritance mental model in general.

I also didn't like having a ton of separate files for all my templates. It
required extra overhead to remember where everything was. The inheritance model
exacerbated this, as I often had to check multiple files to find something in
the chain of inheritance.

I think this could be really cool in a different project. A larger project would
likely benefit a lot from the organization provided by the separate template
files.

### Maud

Instead, I switched to [Maud](https://maud.lambda.xyz/). This is what I was
looking for all along, without knowing it. The HTML I'm generating is simple,
so it fits nicely in the source code of the relevant endpoints. I appreciate
the locality of behavior that this allows.

Additionally, Maud is very composable. I can write templates as functions, and
call those functions within other templates.I can also just run the functions to
render individual sections. It's an absolutely perfect fit for HTMX. I have easy
access to rendering exactly what I need. Nothing more. Nothing less.

Maud syntax isn't similar to HTML. It feels a little more functional in nature.
It kinda reminds me of the [Elm](https://elm-lang.org/) view architecture, but
with Rust style blocks and other syntax. It's very natural to write directly
into Rust code, because it feels very much like Rust, not HTML. Yet, it's still
an easy mental model to map to the underlying HTML.

Here's an example taken from the Maud website:

```rs
html! {
    h1 { "Hello, world!" }
    p.intro {
        "This is an example of the "
        a href="https://github.com/lambda-fairy/maud" { "Maud" }
        " template language."
    }
}
```

Which renders as:

```html
<h1>Hello, world!</h1>
<p class="intro">
  This is an example of the
  <a href="https://github.com/lambda-fairy/maud">Maud</a>
  template language.
</p>
```

It's got tons of really cool syntax features as well. I recommend checking out
the Maud documentation.

---

## Orpheus Architecture

In this section I'll talk a little bit about design decisions that aren't
directly related to the technology choices above.

### State

As required by Axum, Orpheus has a single, central state. It contains several
sub-states that are used by other parts of the code. I wrote a trait `InitState`
that all of these sub-states must implement, and then implemented it on the
central state. This allows me to easily initialize the state all in one place,
while defining sub-state types and initialization behavior in the same files
that use those sub-states. That way I don't have to go to some other file when I
want to check on my state behavior, it's all in the file that uses it.

Right now the main sub-state is storage of the metadata for all the posts. I
wanted to keep this metadata in memory so it could be rapidly filtered and
sorted without requiring disk reads. Actual post text is left on disk until a
user requests it, but metadata needs more instant access--since all metadata
must be loaded in order to sort it.

This does mean that live-reloading new content while the server is running is
impossible, but I'm not worried. At the moment, a server restart with new
content takes less than a tenth of a second, including the container around it.

### Debug Options

There is often behavior I want to change or disable only while debugging, but I
don't want to always perform the same changes just because a debug build is
running. To implement this, I read an environment variable `ORPHEUS_OPTIONS` and
check it for a few strings. For each that is present, I add some specific
middleware.

- no_cache  
  disables all caching by sending a `cache-control: no-store, must-revalidate`
  header. Mostly useful for when rapidly changing css or javascript files, and
  the browser caching behavior is being flaky.
- simulate_lag  
  adds some asynchronous sleep time to every request, to simulate being far from
  the server. This is useful for testing more realistic user experiences than
  the instant feedback I get while loading the page locally. Also nice for
  making sure loading indicators are working properly.
- live_reload  
  injects some javascript into every fresh page load (non-HTMX requests) that
  automatically reloads the page when the server goes offline and then comes
  back.

I can easily set these in the `justfile`, which makes this configuration
convenient.

### Pages

Most pages read some pre-rendered HTML content (and optionally some metadata)
and wrap it in my maud HTML templates as appropriate for the request type
(HX-Boosted vs HX-Request vs no htmx headers).

I wrote an Axum extractor that determines this page type automatically, so that
each page can avoid worrying about that behavior:

```rs
pub async fn posts(page_kind: PageKind) -> impl IntoResponse {
  ...
}
```

This `PageKind` also serves as a page builder:

```rs
page_type
    // page title is required for any page
    .builder("Browse Posts")
    // optional, markup to respond with for htmx requests that are not boosted
    .on_direct_request(post_list_markup)
    // optional page description for meta tags
    .with_description("Browse and filter all blog posts")
    // content is required to "finish" the page, so it is an argument to build.
    // I wanted it at the end of the chain because it is typically the longest
    // part of the code, and this makes it easier to see the other parts
    .build(html! {
        ...
    })
```

This makes it easy to focus on the logic that is specific to each endpoint.

#### Error Handling

There is a special case for pages: errors.

htmx does not like being returned errors of any kind. It will just reject the
response without feedback (by default). However, I wanted to respond with
accurate HTTP status codes for whatever error occurred. Did I need to? No, but
it felt more clean and correct to respond with the right codes.

There is an htmx extension for this, and a few workarounds. However, these either
required cluttering my HTML in ways I didn't want to, or were designed for other
solutions like error toasts. Plus, it felt like this could easily be handled on
the server instead. Why not just do it there?

So, I needed custom error handling to ensure that any request originating from
htmx would receive the correct markup without the incorrect status code.

I accomplished this with some middleware:

```rs
pub fn error_page(
    page_type: PageKind,
    status: StatusCode,
    message: &'static str
) -> Response {
    let markup = page_type.builder(&message).build(html! {
        div
            style={
                "display: flex; flex-direction: column; "
                "justify-content: center; align-items: center; "
                "height: calc(100vh - var(--navbar-height)); "
                "font-size: 200%; "
            }
            {
                span { (message.as_ref()) }
                br;
                a href="javascript:window.history.back();" { "go back" }
            }
    });

    if let PageKind::Boosted = page_type {
        // this is an htmx request, don't send an error code
        markup.into_response()
    } else {
        (status, markup).into_response()
    }
}

pub async fn handle_error_pages(
    page_type: PageKind,
    request: Request,
    next: Next
) -> Response {
    let response = next.run(request).await;

    if response.status() == StatusCode::NOT_FOUND {
        return error_page(page_type, StatusCode::NOT_FOUND, "Not Found");
    }

    if response.status() == StatusCode::BAD_REQUEST {
        return error_page(page_type, StatusCode::BAD_REQUEST, "Bad Request");
    }

    if response.status() == StatusCode::SERVICE_UNAVAILABLE {
        return error_page(
            page_type,
            StatusCode::SERVICE_UNAVAILABLE,
            "Under construction",
        );
    }

    if response.status().is_client_error() {
        return error_page(page_type, response.status(), "Client Error");
    }

    if response.status().is_server_error() {
        return error_page(page_type, response.status(), "Internal Server Error");
    }

    response
}
```

---

# Lyre

This is the preprocessor and build tool I made for use with Orpheus. It is
designed to do as much preprocessing as possible. This was important for a few
reasons:

- keep Orpheus dependency tree as minimal as possible, for faster builds and a
  smaller binary size
- reduce Orpheus startup time as much as possible
- make Orpheus as efficient as possible during runtime
- Lyre can be precompiled in release mode while Orpheus is built in debug, for
  faster iteration speed

---

## Markdown Rendering

Before I even started this project, I knew I wanted to use
[Pandoc](https://pandoc.org/). I could have used one of the many markdown
crates, but none had all the features of Pandoc anyways. Plus, I wanted to
pre-process the markdown, so it's not a huge loss to call an external tool
outside of the build and deploy processes.

Pandoc has an extensive list of input and output formats, with many configurable
extensions for each. This allows a lot of flexibility in my processing pipeline.
For example, at one point I wanted to grab the plaintext version of a markdown
file without any formatting. This was made trivial by adding a second call to
Pandoc with different arguments.

Pandoc also has support for custom filters, either in Lua to run within Pandoc
itself, or by calling external binaries with JSON (de)serialization. I make use
of both kinds of filter in this project.

Some filters are simple, like this one which adds links to headers:

```lua
function Header(elem)
    attr = {
        class = "header-link"
    }
    link = pandoc.Link("#", "#" .. elem.attr.identifier, nil, attr)
    elem.content = elem.content .. {link}
    return elem
end
```

Some are more complex and more powerful, like this one which adds
`<span class="spoiler">` around text that is surrounded by `||` characters:

```lua
function Para(elem)
    attrs = {}
    attrs["class"] = 'spoiler'
    attrs["hx-on:click"] = "this.classList.toggle('revealed')"

    newinlines = pandoc.List()
    collection = pandoc.List()

    for _, c in ipairs(elem.content) do
        if c.text ~= nil then
            s, e = string.find(c.text, "||")
            if s ~= nil then
                before = string.sub(c.text, 1, s - 1)
                after = string.sub(c.text, e + 1)
                if next(collection) == nil then
                    if before ~= "" then
                        table.insert(newinlines, pandoc.Str(before))
                    end
                    table.insert(collection, pandoc.Str(after))
                else
                    if before ~= "" then
                        table.insert(collection, pandoc.Str(before))
                    end
                    table.insert(newinlines, pandoc.Span(collection, attrs))
                    if after ~= "" then
                        table.insert(newinlines, pandoc.Str(after))
                    end
                    collection = pandoc.List()
                end
            else
                if next(collection) == nil then
                    table.insert(newinlines, c)
                else
                    table.insert(collection, c)
                end
            end
        else
            if next(collection) == nil then
                table.insert(newinlines, c)
            else
                table.insert(collection, c)
            end
        end
    end

    if next(collection) ~= nil then
        newinlines = newinlines .. collection
    end

    elem.content = newinlines
    return elem
end
```

Through filters, I can even create my own custom markdown formatting!

I also use a pandoc filter to implement custom codeblock syntax highlighting
using the [syntect](https://github.com/trishume/syntect) crate.

Additionally, some of Pandoc's markdown formats include support for inline
attributes.

For example, the header on the home page of this site looks like this:

```md
# ![](/favicon.svg "logo"){style="max-height: 0.75em"}Drowrin
```

Adding curly braces after the image link allows me to add a style attribute to
the output html. This can be used for any attribute, and there are shortcuts for
commonly used things like classes. `{.spoiler}` sets the class to `spoiler`.
These attributes are added before filters are run, so they can also be used as
configuration for custom filters.

Finally, if I even want to output any of this in another format, I have most of
the work done already. I just need a few different calls to the Pandoc CLI and
I'm done.

---

## Metadata

I wanted to keep post metadata inside the markdown files. Fortunately, Pandoc
has a standard for this: YAML Metadata Blocks. Here's the top of the markdown
file for this article, as an example:

```yaml
---
title: The Making of this Site
published: "2024-05-23"
series: Orpheus Architecture
tagline: Everything Before the Site Went Live
brief: >-
  I've wanted a blog for a while. I've wanted a personal website at the root of
  this domain for a while. I've wanted to explore a handle of web technologies
  for a while. So, it was time to get tinkering.
tags:
  - programming
  - projects
---
I've wanted a blog for a while. I've wanted a personal website at the root of
this domain for a while. I've wanted to explore a handle of web technologies
for a while. So, it was time to get tinkering.
```

The metadata was intended to be fairly self-explanatory and intuitive. So you
can probably guess what each of those elements does.

Most of the elements are optional. Currently the only required fields are
`title` and `published`. I plan to make even these optional eventually.

For all optional fields, Lyre either attempts to infer the metadata elsewhere or
leaves it blank (only if that is a reasonable outcome). For example, a `tagline`
is left blank by Lyre if the author does not set it, because Orpheus will simply
omit the tagline from the HTML if it isn't present. On the other hand, a `brief`
is important for multiple things, including meta tags and search results. So if
there is no `brief` field, Lyre will use the first paragraph of the document
with all formatting symbols stripped.

Lyre gathers more metadata than is provided by the frontmatter block, as well.
For example, the word count and reading time are measured based on a plaintext
render of the document. The URL slug for the article is also generated.

In the future, I'd like to make this even more automatic. A big feature I'd love
is the ability to infer `published` and `updated` by reading the git history for
each file. It may also be possible to suggest tags based on document content.

---

## Javascript Bundling

As I started to add more CDN links to my `<head>` for javascript, I realized it
was time to start bundling and serving it myself instead. My reasoning was as
follows:

- If I pack all the javascript into one file, I'll get better compression savings.
  Fewer total HTTP requests is also more efficient for the client.
- The extensions for htmx are not minified by default, even though there is a
  minified version of htmx itself. This isn't a big deal, but I might as well
  get those savings with a bundler.
- Having the dependencies installed and bundled means I can pin dependency
  versions in `package-lock.json` and let Dependabot handle version upgrades. This
  comes with the downside of requiring npm to build the project, but I really
  appreciate automatic dependency updates, so I think it's worth it.
- Earlier in the project, I had a few more javascript dependencies, so this was
  even more worth it. However, I still think it has benefits even for the more
  lean dependency set I have now.

I decided to go with [Parcel](https://parceljs.org/). It seemed like the
simplest tool for the job. I'd love to find a Rust crate to handle this, but at
least Parcel can be installed through npm (which is required anyways).

---

## Style Preprocessing

I did find a crate for this one: [grass](https://docs.rs/grass/latest/grass/).

Not too much to say about this one. Grass runs Sass to CSS processing.

I'm still a Sass newbie, so there's a lot of room for improvement.

---

## Hashing and Caching

I don't want to run all of Lyre every time something changes. Some steps of the
process can take a handful of seconds each. Sometimes Parcel gets stuck and
takes 10x the time as usual. Ideally, Lyre would only process inputs that
changed since the last run. This saves time and allows live-reload to be as
snappy as possible.

So, Lyre emits a `.hash` file for each step of the process, based on the input
files for that step. When it gets to each step, it will skip if the hash of the
current input files matches the saved `.hash` file.

There's lots of room for improvement here. Currently, this is scoped to each
step, not to each input file. This works well for plenty of the steps, but some
could really use some more granularity. As I add more posts, the markdown
rendering step continues to get longer. In the future, I'd like to allow Lyre to
output hashes for each of the input files for each step. This way

I'd also like to allow multiple threads for some of the steps. The steps
themselves don't usually depend on each other, so most could be run in parallel
right from the start. Additionally, most files processed by each step are
independent of each other.

---

## Lyre Architecture

Lyre has several companion crates:

Verse is a crate containing serde structs for all the files saved by Lyre. That
way, other crates in the project can easily read these files in a type-safe way.

Melody is a trait and utility crate that abstracts out the common behavior of
Lyre steps. It is essentially a framework used only by Lyre. The abstraction is
mainly for faster build times, but also in theory other crates could use it to
check the status of input file hashes and such.

All outputs from Lyre are placed in a `./generated` directory. Most inputs come
from the `./content` folder, but Lyre also reads from `node_modules` for the
javascript and css steps.

Here is an example Melody step:

```rs
struct Pages;
impl Melody for Pages {
    // used for logging a name for the .hash file
    fn name() -> &'static str {
        "Pages"
    }

    // an iterator of input files
    fn source() -> eyre::Result<_> {
        in_dir_with_ext("./content/pages", ".md")
    }

    // an iterator of output files
    fn rendition() -> eyre::Result<_> {
        Ok(Self::source()?.into_iter().map(|p| {
            let p: PathBuf = p.into();
            Path::new("./generated/pages/")
                .join(p.file_name().unwrap())
                .with_extension("html")
        }))
    }

    // the action taken to produce output files
    fn perform() -> eyre::Result<()> {
        for path in Self::source()? {
            ...
        }

        Ok(())
    }
}

fn main() -> eyre::Result<()> {
    // handles hashes, creating missing directories, logging status, etc
    <Pages as Melody>::conduct()?;

    // other steps go here
    ...

    Ok(())
}
```

By using Melody, the code for each Lyre step only needs to contain details for
its specific implementation. All other common behavior is handled by Melody.
This makes it very easy to make changes to the behavior of all Lyre steps at
once.
