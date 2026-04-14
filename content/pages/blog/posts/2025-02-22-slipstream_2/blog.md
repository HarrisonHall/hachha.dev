You heard it here first, `slipstream` is
[_out_](https://github.com/HarrisonHall/slipstream/releases/tag/slipstream-1.0.0),
just in time for
[cat day](https://en.wikipedia.org/wiki/National_Cat_Day#Japan)!

![slipstream](/blog/slipstream_2/web_ui.png)

I couldn't be happier with the result, but it's worth noting this isn't what I
originally promised. Where are custom lua filters? Tracking read articles (read:
headlines)? Super fancy tui?

After using `slipknot` for a while, I realized I didn't actually care about many
of those features. If I need a new filter, I can just push a new version of
slipstream out. My readers can track what I've read, and I no longer care about
sharing that between devices.

So what happened to `slipknot`? `slipstream` now contains all of what was
`slipknot`. I didn't see a reason to keep them separate or reimplement features.
`slipstream` is basically `slipknot` with the default addresses going to the web
view (atom feeds are now accessible with an extra `/feed` in the path).
Honestly, I felt the name `slipknot` was a little aggressive, I wanted something
with "slip" in it, but didn't think too much about it.

But seriously, [check it out](https://feeds.hachha.dev/)! The source remains on
[github](https://github.com/HarrisonHall/slipstream).

## Future Plans

I may still revisit my own tui in the future, but for now `newsraft` (tui) and
`feeder` (mobile) are completely sufficient for my own needs.

There are some outstanding tasks I need to eventually finish up.

- `slipfeed`
  - [ ] Add other built-in feed implementations (e.g. activitypub)
- `slipstream`
  - [ ] Add more filters (regex/pomsky, allowlists, etc.)
  - [ ] OPML conversion support
  - [ ] Use sqlite for storing entries and feed definitions
  - [ ] Support atom exports

...but I don't need any of these now, so who knows when they'll be completed.
¯\\\_(ツ)\_/¯
