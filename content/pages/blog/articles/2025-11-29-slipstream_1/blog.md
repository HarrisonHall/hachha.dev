I've been a proponent of RSS/Atom feeds for a long time. While I discovered them
in college, I hadn't embraced them until a couple of years later. I've moved
between readers, importing/exporting opml files as my collection of feeds grew,
but eventually landed on [Feeder](https://github.com/spacecowboy/Feeder) as a
solid, good-enough mobile reader. After a while, I realized that I wanted a bit
more control over my feed. Why should I see content that I don't care about?
Sports from NPR? Ads from The Verge? These questions ignited the second phase of
my journey.

## Miniflux

When I had finally set up my home server earlier this year, I was finally free
to host my own "read-it-later" service. After a bit of research, I decided
[Miniflux](https://github.com/miniflux/v2) was a good bet. Miniflux not only
allows you to add filters and tags to your feeds, but it also tracks what you
read. It's great that I can scroll through the web interface and know that after
I finish the articles shown-- there's nothing left. I've reached the end. Since
then, I've gladly used the service to view my daily news.

What Miniflux _doesn't do_ is allow me to re-export my feeds. It's great that
the interface is password-protected, but the mobile web interface was far from
perfect. There's a great looking associated mobile app, but it _ate_ data. It
would have been incredibly convenient if I could re-export my feeds as something
I could subscribe to through feeder.

Miniflux does allow filters a la blocklists, but it doesn't allow aggregate
feeds outside of tags. And while regex blocklist are most of what you want, they
aren't everything. Miniflux has a useful rules system, but it wasn't quite what
I was looking for. What I was looking for, was _Yahoo Pipes_.

## Yahoo Pipes

I was never lucky enough to use Yahoo Pipes as it only lasted from 2007-2015.
But when it was around, it provided a simple interface for creating aggregate
feeds from a variety of sources while allowing users to perform operations on
the data between pipes. The "pipes" theme was a reference to unix pipes,
implying users would easily be able create new feeds like chaining commands.

As I said, Yahoo Pipes doesn't exist anymore. And even if it did, you wouldn't
host it yourself. This (and a couple of free weekends) led me to start
developing slipstream.

## Slipstream

[Slipstream](https://github.com/HarrisonHall/slipstream) is composed of a few
crates (`slipfeed`, `slipknot`, `slipstore`, and `slipstream`)-- but right now
the important one is `slipknot`, a simple CLI application that periodically
fetches, filters, and re-exports feeds defined in a TOML file as new atom feeds.

```toml
update_delta_sec = 7200

[global.filters]
exclude-title-words = [
  "ai",
  "llm",
]

[feeds.hacking]
feeds = ["hackernews", "ziglang"]

[feeds.hackernews]
url = "https://news.ycombinator.com/rss"
tags = ["tech"]
exclude-title-words = ["hiring"]

[feeds.ziglang]
url = "https://ziglang.org/devlog/index.xml"
tags = ["blog", "zig"]
```

While still a work-in-progress, running something like
`slipknot --config that_file.toml --port 13300` will start a web server that
exposes the following endpoints:

- `/config` for viewing the config.
- `/all` for viewing all entries.
- `/feed/<feed_name>` for viewing a specific feed.
- `/tag/<tag_name>` for viewing a feed for entries with a specific tag.

There are `RawFeed`s (comprised of a URL) and `AggregateFeed`s (comprised of
other, existing feeds). Each feed can have its own filters (slipknot exposes
`exclude-title-words` and `exclude-content-words`, currently), but there are
also global filters.

I am pretty content with the progress so far (for my use-case, it's already 90%
usable!), but I do have much bigger plans as outlined by the roadmap on github.
Eventually, slipknot will have a bigger, younger brother named slipstream (hence
the repo name) that will have a web interface similar to Miniflux with
persistent storage powered by `slipstore`. There's no ETA on this, as with most
of my projects there's a good chance it'll die when it becomes usable for my own
standards. Regardless, I'm excited.
