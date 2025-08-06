# <img src="content/favicon.svg" width="24" /> Orpheus

My custom site generator.
It's not really intended for general use, as it's very tightly coupled to my data.
So the data for my site is also included.

It's designed to produce a lean, minimal site--but the backend grows ever more overengineered.
It could be simpler, but I'm using this as a place to tinker with technologies I'm interested in,
even if they might be overkill.

My intention is to not compromise on the features I want for the site, while keeping it as efficient as I can.

Check out the live site at [drowrin.com](https://drowrin.com)

## Features

- process markdown through pandoc for extremely powerful extensions and custom filters
- auto-selected, user switchable themes based on Catppuccin Mocha and Latte
- pre-process as much as possible before the server ever goes live, through an included companion tool, lyre
- cache-busting parameters in release mode, live-reloading and disabled caching in dev mode
- all live content sources isolated in a single directory, `content`
- avoid as much client-side work as possible
- automatic table of contents
- preload page content and images when a user hovers over a relative link
- automatic publish date and revision history from git commit messages

## Requirements

This project uses [mise-en-place](https://mise.jdx.dev/) for dependency management.

Run `mise install` to download dependencies, then run `mise tasks` to view all tasks.

## Organization

All Rust code is in `src/`, split into three crates. Scripts are in `.mise/tasks/`.
Post and page sources, images, and all web content (CSS, JS, and so on) is in `content/`.

### Orpheus

The main project. This is the backend server code, with all the templates,
logic, and state management. It's got code for intercepting HTMX headers and
rendering the appropriate responses, as well as endpoints for filtering and
searching blog posts.

### Lyre

This used to handle all of the rendering, but that's all handled by scripts now.
Currently, Lyre creates the search index. It will likely be removed when I change
to another search solution.

### Verse

A library containing common data formats between the other project parts. Generally used
for serde struct definitions for custom files, so that Orpheus can read a file that Lyre
generated.

## Technology Used

This site is primarily a place for me to tinker with various technologies I'm interested in.
I've listed below what I'm currently using, and why I'm using it.

### HTMX

This lets me control a lot of behavior through HTML attributes alone.
The backend of this project responds primarily in composable sections of HTML.
HTMX takes care of efficiently swapping these sections of HTML when they arrive.
The browser no longer needs to accept a JSON/Protobuf payload, deserialize it,
and render it into HTML--it just gets HTML directly. Is this inefficient for the
server? In my experience, no, because it's really not any harder for the server to
run a string builder than to serialize JSON, and compression handles HTML extremely well.

The only other frontend code I use is a few lines for theme switching,
and a few helper functions to pass into HTMX attributes.

### Rust - Axum

Putting these all together because they're pretty linked. Rust is a language I've
always enjoyed using, but I've never applied it to a more longterm project like this.
I wanted to change that, so here we are. I've found it fantastic for this purpose.
I really like how Rust allows me to get a lot out of functional programming patterns
if I choose to use them, and the type system is the best I've ever used.
Axum is an extremely convenient and ergonomic web framework.

### Maud

I tried a few templating engines, but fell in love with this one as soon as I tried it.
All your templates are just functions in the source code. This makes templates
easily composable through ergonomic language features. I strongly prefer this style
compared to templating systems with inheritance built in. I also appreciate that
templates being part of my source code improves locality of behavior.

It's not all perfect though. Changes to templates mean re-compiles. This can be a little
annoying, but I've managed to minimize it. With my setup, live-reload takes under a second
from saving the file. Also, while I love Maud's custom syntax, it makes it difficult to
run other tools that analyze html.

### Pandoc

Numerous markdown -> HTML solutions exist. I've tried them. None beat the flexibility of
Pandoc. I've written several custom filters in both Rust and Lua for this project. I can
implement any feature I can dream of using that feature. It's extremely powerful.

Plus, I'm pre-rendering all of my markdown into HTML before the server ever starts up,
so having a parser as a crate isn't that important to me. Still, I'm using several crates
to assist in working with Pandoc from Rust.
