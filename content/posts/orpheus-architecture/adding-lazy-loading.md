---
title: Adding Lazy Loading
tagline: Why Can't I Hold All These Posts?
series: Orpheus Architecture
tags:
  - projects
  - programming
brief:
published: 2024-05-24
updated:
---

I don't yet have **THAT** many posts, so this hasn't been a problem yet--but I
think it would become an issue eventually: Loading all the posts at once in the
post browser is a bit overkill. It slows down the delivery speed of results,
which makes the interface slower, and also impacts page loading times in
general.

Fortunately, there's a solution for that. Infinite Scroll is a very popular
pattern for a reason. It's pretty intuitive and doesn't require any special
input from the user.

I'd never implemented something like this before, so I was curious how tricky it
would be. I know many frontend frameworks have components and/or extensions for
this, but how would it work with htmx? Would I have to write some dreaded
client-side javascript? Would I have to watch for changes in the scroll
position?

Turns out, htmx makes this _extremely easy_.

# Backend Changes

I figured that before I even started tinkering with htmx, I'd need to allow some
simple pagination of the posts. I added a new field to my Query struct:

```rs
#[derive(Clone, Deserialize, Serialize)]
pub struct PostsFilters {
    #[serde(default)]
    tag: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    // 👇 this is the new stuff
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<usize>,
}
```

> **Note**:  
> All of the `#[serde(skip_serializing_if = "Option::is_none")]` lines just tell
> `serde` to completely omit those fields when empty, instead of serializing
> them like `?series=&search=&skip=`. I find that pretty ugly, so this is the
> fix. If I was _only_ deserializing this struct, I could skip that
> configuration, but I serialize it in a few places in order to add queries to
> my htmx URLs.

Adding those two lines is all that was required to have my backend look for the
`skip` query parameter and pass it to my `/posts` endpoint. Then I just had to
make sure to apply it to my filtering process:

```rs
let mut filtered_posts: Vec<&PostMetaData> = posts
    .metadata
    .values()
    .filter(|m| {
        if let Some(series_slug) = &query.series {
            match &m.series {
                Some(series) if series.slug.eq(series_slug) => (),
                _ => return false,
            }
        }
        if let Some(search) = &query.search {
            if !m
                .title
                .to_lowercase()
                .contains(search.to_lowercase().as_str())
            {
                return false;
            }
        }
        query.tag.iter().all(|t| m.tags.contains(t))
    })
    .collect();

filtered_posts.sort_by(|a, b| b.published.cmp(&a.published));

// 👇 this stuff is new
let skip = query.skip.unwrap_or(0);
filtered_posts.drain(0..skip);
filtered_posts.truncate(CHUNK_SIZE);
```

> **Note:**  
> `CHUNK_SIZE` is a global const I set to 5. I may make this configurable in the
> future, but after some testing 5 seemed to strike a reasonable balance. It
> doesn't increase the payload size much, thanks to compression handling the
> repeated patterns so well. It also feels pretty good to scroll without too
> many interruptions.

After a couple quick tests by manually adding the query to the url, I can see it
works!

# HTMX Changes

So now that we've got filtering working as expected, it's time to figure out how
to get htmx to load it.

**One attribute is all it takes.**

Well, sorta. It takes 3, but one does _most of the magic._

```rs
div
    hx-get={"/posts?" (serde_html_form::to_string(query)?)}
    hx-swap="afterend"
    hx-trigger="revealed" // 👈 this is the magic one
    { (post_card(post)) }
```

When that `div` enters a user's viewport, it will trigger a request to `/posts`
with the current search query. When it receives new HTML from the request, it
will place it after the `div` that triggered the request.

I plugged that into my current code, and it also works!

Obviously it doesn't yet do what I actually want--but it's close! I just wanted
to see the htmx part of the code work before I got back to fixing other things.

The main issue with this is that it doesn't yet add the `skip` parameter, so it
just repeats the first 5 posts as you scroll. We've got all the building blocks
ready to fix that:

```rs
let skip = query.skip.unwrap_or(0);
let new_query = PostsFilters {
    skip: Some(skip + CHUNK_SIZE),
    ..query.clone()
};

// ...

div
    hx-get={"/posts?" (serde_html_form::to_string(new_query)?)}
    hx-swap="afterend"
    hx-trigger="revealed"
    { (post_card(post)) }
```

The other issue is that not every post needs this. Let's only add it to the last
post of each batch:

```rs
let posts_markup = html! {
    @if filtered_posts.len() > 0 {
        @for post in &filtered_posts[0..filtered_posts.len()-1] {
            div { (post_card(post)) }
        }
        @if let Some(post) = filtered_posts.last() {
            div
                hx-get={"/posts?" (serde_html_form::to_string(new_query)?)}
                hx-swap="afterend"
                hx-trigger="revealed"
                { (post_card(post)) }
        }
    }
};
```

This works great! Scroll works. New page loads work (and automatically fetch
more posts if the browser window is tall enough).

I'm not perfectly happy with it yet though. This isn't a true "Infinite Scroll",
as I do not have infinite posts on this site. It ends at some point. With the
current design, an extra request is fired to fetch empty content. There's no
reason to send that last request. Let's fix it:

```rs
filtered_posts.sort_by(|a, b| b.published.cmp(&a.published));
filtered_posts.drain(0..skip);
// 👇 this line is new
let more_after = filtered_posts.len() > CHUNK_SIZE;
filtered_posts.truncate(CHUNK_SIZE);

// ...

let posts_markup = html! {
    @if filtered_posts.len() > 0 {
        // 👇 this branch is new
        @if more_after {
            @for post in &filtered_posts[0..filtered_posts.len()-1] {
                div { (post_card(post)) }
            }
            @if let Some(post) = filtered_posts.last() {
                div
                    hx-get={"/posts?" (serde_html_form::to_string(new_query)?)}
                    hx-swap="afterend"
                    hx-trigger="revealed"
                    { (post_card(post)) }
            }
        } @else {
            // 👇 this part is also new
            // if we don't have any more content after this batch,
            // don't send infinite-scroll attributes
            @for post in filtered_posts {
                div { (post_card(post)) }
            }
        }
    }
};
```

That's it! Fully functioning infinite scroll added to my current search/browse
page. All with _3 htmx attributes_ and a small amount of painless logic in Rust.

I'm pretty happy with it. I'm really enjoying working with this stack.
