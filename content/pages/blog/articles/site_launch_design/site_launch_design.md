## Hello, World!
I launched a new site with a new changes:
- De-emphasized resume
- A blog (with an [rss](https://hachha.dev/blog.rss) feed)
- Streamlined design

Most of this is subject to change as most of the design
isn't finalized. I even phoned it in on the last few bits
(resume and games pages), so those will probably exist in
_some_ form in the future.

My last personal site has been updated to redirect here
([harrisonchristianhall.com](https://www.harrisonchristianhall.com)).
The site content should still be viewable in some form for the time
being on (github)[https://github.com/HarrisonHall/harrisonhall.github.io].

Updating my old website was a pain. Static webpages
(with any nontrivial styling) 
aren't easy to update manually. I wanted a system where I could update
content in something markdown pretty easily. I also wanted to be able to
template different parts of the page(s) for easy development. I figured
I could do most of this using the platforms github provides for personal
sites, but I also wanted to be able to fancy things in the future that
you just can't without your own server.

## Rust
I'm always itching for a new project to practice my rusty rust skills.
Since I hadn't created a webserver in rust yet, I thought I'd try.

I looked at the current state of rust webservers and decided to go with
axum for its 1) `tokio` integration, 2) `async` support, and
3) 2023 support.

Ultimately, the site builds into a simple struct at launch.
If there are any gotchas (`panics`), they should happen during the creation
of this struct or while attempting to bind to the specified port.
As far as the site struct goes, I only expect `panic`s to occur
when creating the `Pages` struct by parsing embedded files.

```rust
pub struct Site<'a> {
    pub config: SiteConfig,
    pub templater: Handlebars<'a>,
    pub pages: Pages,
    pub page_cache: Cache<Html<String>>,
    pub content_cache: Cache<Vec<u8>>,
}
```

The entire project is designed to compile into a single binary- content
included. The CLI provided allows for specifying things like the port,
certs, logging, cacheing, etc.

### Cache
Templating the same information onto the same page over and over is a
cpu-time-waster. I developed a 
[cache](https://github.com/HarrisonHall/hachha.dev/blob/master/src/cache.rs)
system that can be used by different routes to check if the page has
already been filled out recently. You can configure the cache timeout on
the command-line, but the default is 5 minutes.
The cache struct has safe async support built in, so it can be called
by different routes handling different requests at the same time.
If some page needs to be rendered every time, I just won't deal with
the cache at all.

```rust
/// Cache
pub struct Cache<T> {
    /// Statefull entries
    entries: RwLock<HashMap<String, CacheEntry<T>>>,
    /// Time until an entry expires (in seconds)
    timeout: f32,
}

/// Entry inside of cache
struct CacheEntry<T> {
    entry: T,
    update_time: Instant,
    timeout_override: Option<f32>,
}
```

## Page Design
### Index
Most top-level pages are like the index-- effectively a static web page.
Some information is templated in (like the current year for the footer),
but all-in-all these pages mostly just take advantage of the templating
system provided by [handlebars-rust](https://github.com/sunng87/handlebars-rust).

### Blogs
I don't actually plan on blogging much (if at all), but I _really_ wanted to
make a system for doing it. This system consists of a single manifest that
lists the metadata for the articles and a series of folders with an
`articles/{article_name}/{article_name}.md` layout. The article folders can
also contain arbitrary data such as media.

### Projects
The projects page consists of a select few projects described in a yaml file.
There isn't too much to describe.

### Misc.
Other top-level pages in the header currently link to pages outside of
hachha.dev. This should change in the future, but motivation comes and
goes and this isn't important enough to waste energy on.