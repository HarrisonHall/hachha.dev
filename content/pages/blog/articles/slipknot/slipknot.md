Slipknot has grown legs! Not wings, but legs.

The [slipstream](https://github.com/HarrisonHall/slipstream) crate `slipknot` is
finally _usable_ with version `0.3.1`. In short, this means that realistic
configurations work well enough that `slipknot` has replaced my reliance on
miniflux!

```toml
# slipknot example config

# Root settings for the updater.
freq = "2hr"
cache = "2min"

# Settings that apply to **everything**.
[global]

[global.filters]
exclude-title-words = [
  "llm",
]
exclude-content-words = [
  "javascript",
  "node",
  "nodejs",
]

[global.options]
# Maximum feeds returned, something sane.
max = 512
# Oldest entry stored.
oldest = "1month"

# Settings that apply to the all feed.
[all]
exclude-title-words = [
  "release",
]

# Feed definitions.
[feeds]

[feeds.hacking]
feeds = ["hackernews", "ziglang-compilation"]

[feeds.hackernews]
url = "https://news.ycombinator.com/rss"
tags = ["tech", "news"]
exclude-title-words = ["llm", "hiring"]
freq = "30min"

[feeds.ziglang-compilation]
url = "https://ziglang.org/devlog/index.xml"
tags = ["blog", "zig", "tech"]
include-substrings = ["llvm", "compilation", "binary", "optimization"]

[feeds.nhk]
url = "https://www3.nhk.or.jp/rss/news/cat0.xml"
tags = ["news", "japanese"]
max = 5
```

The example config above (from the `slipstream` repo) works well enough. My
[personal configuration](https://github.com/HarrisonHall/dotfiles/blob/master/dotfiles/.config/slipknot/slipknot.toml)
now has 68 feeds! `slipknot` supports log files, varying request rates, and
utilizes the `If-Modified-Since` HTTP header.

I'm continuing to use `slipknot` with
[`newsraft`](https://codeberg.org/grisha/newsraft) locally, but I plan on
creating my own tui viewer as part of `slipstream`... eventually.
