# {{icon hammer}} Colophon

_IMO_, this site is _wicked cool_.

This site is completely custom. At its core, this is all just
[`axum`](https://github.com/tokio-rs/axum) served over
[`caddy`](https://caddyserver.com/). The
[source](https://github.com/HarrisonHall/hachha.dev) is available.

Some of the cooler, custom features are:

- A caching system
- Embedded & bundled data
- Atom feed generation
- Blog parsing & tagging
- [Indieweb](https://indieweb.org/) spec integrations
  - [h-card](https://indieweb.org/h-card)
  - [h-entry](https://indieweb.org/h-entry)
  - [slashpages](https://slashpages.net/)
- Holiday themes
- ...
- There's some other cool stuff, I promise

"Custom" as in, "I coded that." Nothing in this site is particularly novel, but
it is fun to hack around with. I wrote [a few](/blog/site_launch_design) older
blogposts about it.

As far as the rest of the stack goes:

| Tech         | Framework                                                                       |
| ------------ | ------------------------------------------------------------------------------- |
| Icons        | [Phosphor](https://phosphoricons.com)                                           |
| CSS          | [PicoCSS](https://picocss.com/)                                                 |
| Highlighting | `highlight.js`                                                                  |
| Templating   | `Handlebars`                                                                    |
| Markup       | Markdown, via `Comrak`                                                          |
| Frontmatter  | `Toml`                                                                          |
| Fonts        | MPLUS1, Fira Sans                                                               |
| Crates       | [`Cargo.toml`](https://github.com/HarrisonHall/hachha.dev/blob/main/Cargo.toml) |
| `robots.txt` | [ai.robots.txt](https://github.com/ai-robots-txt/ai.robots.txt/tree/main)       |
