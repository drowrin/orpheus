# <img src="content/favicon.svg" style="max-height: 0.75em" /> Orpheus

My custom site generator.
It's not really intended for general use, as it's very tightly coupled to my data.
So the data for my site is also included.

It's designed to produce a lean, minimal site--but the backend grows ever more overengineered.
It could be simpler, but I'm using this as a place to tinker with technologies I'm interested in,
even if they might be overkill.

My intention is to not compromise on the features I want for the site, while keeping it as efficient as I can.

I'm still working on setting up the site before I fill it with content. I have more local content that
I use to test the site, but it isn't polished enough to commit to the live site yet. I do have at least
one post that I think is ready at this point, so it is included as an example. This is still very much a work
in progress project.

## Features

- process markdown through pandoc for extremely powerful extensions and custom filters
- auto-selected, user switchable themes based on Catppuccin Mocha and Latte
- pre-process as much as possible before the server ever goes live, through an included companion tool, lyre
- cache-busting parameters in release mode, live-reloading and disabled caching in dev mode
- all live content sources isolated in a single directory, `content`
- avoid

### Roadmap

- read git history to determine post publication/update dates
- more efficient caching scheme
- rss feed generator
- opengraph tags
- "related posts" section
- image format content negotiation

## Requirements

This project uses a bunch of disorganized tools that I find useful.
Perhaps someday I'll learn Nix and add a flake for this project, but for now
several things must be installed locally for the project to run.

- [pandoc](https://pandoc.org/)
- [just](https://github.com/casey/just)
- [npm](https://www.npmjs.com/)
- [cargo-watch](https://crates.io/crates/cargo-watch)
- [cargo-shuttle](https://crates.io/crates/cargo-shuttle)

### Installation

```sh
npm install
cargo build
cargo build -p lyre --profile release
```

See the [shuttle documentation](https://docs.shuttle.rs/getting-started/quick-start)
to set up deployment

### Usage

```sh
# live-reload the project as you make changes
just watch
# render/generate all content without running the server
just lyre
# delete all lyre-generated content
just clean
# render/generate content and then deploy to shuttle
just deploy
```

See the [justfile](/justfile) for all recipes.

## Organization

This project is split up into 4 parts, and a few directories for non-rust code.

### Orpheus

The main project. This is the backend server code, with all the templates, logic,
and state management. It's got code for intercepting HTMX headers and rendering
the appropriate responses. It's got a bunch of middleware to attempt to apply as
many best practices as I'm aware of. It's got endpoints for filtering and searching
blog posts.

### Lyre

The companion tool. This tool handles rendering, generating, and caching static content
for Orpheus to read. This includes markdown -> HTML, bundling javascript, running SASS,
collecting metadata, creating plaintext versions of posts for use elsewhere, and creating
hashes to be used in caching its own work as well as cache-busting in Orpheus. It's my
all-in-one build tool for everything cargo doesn't handle.

### Verse

A library containing common data formats between the other project parts. Generally used
for serde struct definitions for custom files, so that Orpheus can read a file that Lyre
generated.

### Melody

A trait and utility library used by Lyre. It provides a framework for build tasks with common
logic. All a Lyre task needs to supply is a list of input files, a list of output files, and
a function to run to accomplish that goal. Melody handles the rest: caching, hashing, timing,
logging, (de)serialization, ensuring output directories are created, and more. It also provides
utility functions, such as "my list of input files is all markdown files in ./content/posts".

### Pandoc

All my Lua pandoc filters live in one directory, `/pandoc`. They are separated out into a single
file each so that they are easier to add/remove from different rendering pipelines.

### Web

This directory contains all my CSS and JS code. It's not much, but it's enough that a separate
directory was a good idea. None of this gets served as-is. It all gets processed through SASS
and a bundler and placed in `/generated/static`.

### Content

This is the only place actual content lives. Even the favicon lives here. Most of this gets
processed into some other form by lyre before being served. The exception is the `img`
subdirectory. Currently, Orpheus serves directly from `/content/img` to save storage space.
This may change in the future with the addition of more image formats and content negotiation.

### Generated

All content created by lyre will go in the `/generated` directory. This is the primary directory
that Orpheus reads from. It is ignored by git, as it is essentially full of build artifacts.

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

### Rust - Axum - Shuttle

Putting these all together because they're pretty linked. Rust is a language I've
always enjoyed using, but I've never applied it to a more longterm project like this.
I wanted to change that, so here we are. I've found it fantastic for this purpose.
I really like how Rust allows me to get a lot out of functional programming patterns
if I choose to use them, and the type system is the best I've ever used.
Axum is an extremely convenient and ergonomic web framework. Shuttle has been a
very convenient place to deploy the site.

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

### Pico CSS

I tried a few different CSS frameworks (including Tailwind) before settling on this.
The ability to get a good looking site based on semantic HTML alone is very pleasant.
It's also easy to configure, and I can step out of it with inline styles or custom css
where required (though I haven't found many places this was needed). It helps that Pico's
defaults are very close to what I'd want anyways.

### Syntect

This is the crate I settled on for code highlighting. I wanted it to be done in the backend,
not deferred to client-side javascript. Unfortunately, I found Pandoc's built in highlighting
to be a little insufficient. Fortunately, I was able to use this crate as a Pandoc filter fairly
easily. So now I've got HTML5 compliant code blocks, highlighted in custom themes with excellent
language support. Plus it's all part of the same render step as the rest of the html, and not a
second pass.

The only downside is that the CSS classes are a little bulky. It's brought my CSS up to about 20kB.
I might be able to perform some SASS tricks, and/or remove some languages I'm unlikely to use
to bring this down in the future.

### Parcel

This seemed like the simplest bundler for the job, and it works great.
Unfortunately, HTMX needs a bit of a hack to work with bundlers, but it isn't a big deal.
I wanted to use a bundler and include npm in the project so that I could take advantage
of dependency version watching tools like Dependabot. It also means I serve a little less
JS overall, and in a single file that compresses better.

Despite this being an external tool, I run it from lyre as part of the build process.
This is definitely the longest build step, but I cache it and it rarely changes.
