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

Ultimately, the site (will) build into a simple struct at launch.
If there are any gotchas (panics), they should happen during the creation
of this struct or while attempting to bind to the specified port.

```rust
pub struct Site<'a> {
    pub config: SiteConfig,
    pub templater: Handlebars<'a>,
    pub blog_indexer: BlogIndexer,
}
```

The entire project is designed to compile into a single binary- content
included.

## Design
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
Todo `¯\_(ツ)_/¯`