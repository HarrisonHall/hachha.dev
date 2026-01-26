## Motivation

A couple of times a year, a new season of an anime comes out that I would like
to see, so I'll go ahead and get the appropriate streaming subscription for a
month or two. But since I respect the work itself, I want to actually sit down
and watch the show. This means I can't really watch it while cooking or eating
or doing other chores. Since I want to actually give my attention to it, _I
almost view it as a kind of work_. A work worth doing, but still work. So in
practice, I end up watching a couple of campy shows before moving onto what I
actually wanted to watch.

Campy shows vary greatly. I have seen a few handfuls of campy isekais, and I
generally, genuinely enjoy them. But it seems about 1 in 3 are just actually
soulless. What I think I mean by that is that for me to enjoy one of these campy
fantasy animes, I have to be able to appreciate some form of artistry within it.
It can be formulaic, but it needs to stand out (just slightly) in some way in
terms of story, characters, world-building, animation, music, etc. Depending on
the level of artistry, these shows can last somewhere between 0.5-2 seasons
before growing tiring.

I find books themselves to be generally more active than a show I can put on
while cooking dinner. For something to hold my attention it needs to generally
be interesting and written for and audience like myself. This makes learning
Japanese more difficult than it should be because it is difficult to find
material that satisfies these constrains _and is accessible_. I'm a big fan of
the
[てんスラ (tensura)](https://en.wikipedia.org/wiki/That_Time_I_Got_Reincarnated_as_a_Slime)
franchise, but the LNs are simply beyond my current reading level. LNs typically
on my reading level (e.g.,
[くまクマ熊ベアー(Kuma Kuma Kuma Bear)](https://en.wikipedia.org/wiki/Kuma_Kuma_Kuma_Bear))
do not hold my interest.

While possible to power-through, looking up every other word, there are tools to
help with this process. [Yomitan](https://yomitan.wiki/) adds lookup right to
your browser and [LingQ](https://www.lingq.com) is an entire platform designed
around the reading experience. Using LingQ has some drawbacks. I must be using a
phone or pc to actually read. **What I really want is to be able to read tensura
on my e-reader without worrying about word lookup.**

## durf

I like reading articles from my rss/atom feeds. I have a workflow where I can
curl an article, convert it to markdown in pandoc, pipe it to bat for styling,
and read the ANSI output all within my
[slipstream reader](https://github.com/HarrisonHall/slipstream). This usually
works great, but I've been thinking about taking a more-native approach.

Ideally, I'd like to have a sort-of "Firefox reader view" widget that I can
embed into slipstream reader that supports custom styling and follows links.
That's what `durf` is (or will be). `durf_parser` is the component of `durf`
that parses an html document and minimizes it to essentially only include the
contained text. It was a side project I worked on a few months ago but haven't
touched since. The parsing and minimization into a proprietary AST was roughly
complete, just dirty and unpublished.

Last week I was doing some reading in my browser and thought, _"I wonder if I
can annotate the text from a web novel I'm reading."_

## jdpub

`jdpub` works by reading/fetching the HTML, minimizing the structure into a
`durf` AST, iterating the text fragments, tokenizing the fragments, searching
each token up in a database, annotating the token as a new fragment
appropriately, rebuilding the document into XHTML, and inserting the XHTML into
an EPUB file. It (currently) uses a JLPT level to determine if words should be
annotated with the kana, definition, and JLPT level.

![`jdpub` being used on the tensura web novel](https://raw.githubusercontent.com/HarrisonHall/jdpub/refs/heads/main/metadata/media/example.png)

`jdpub` is _usable_. The current version is `0.2.0`-- and it definitely feels
that way-- but it is absolutely possible to parse a webpage. Check it out on
[github](https://github.com/HarrisonHall/jdpub) and give it a try.

Sadly, since `durf` hasn't been released yet, you can't actually build it
yourself. You'll have to trust the release is legitimate.

By default, `durf` will parse everything in the page, but it supports a
configuration file that allows you to manually specify `allow` and `skip`
selectors based on element type and class. I plan to extend this, but as-is this
was a feature I had to add to `durf_parser` in order to support `jdpub`
(otherwise the books had a bunch of junk artifacts in the front matter and back
matter).

I'm hoping this tool will help motivate me to read more-- I already feel like
I'm behind by spending 10 hours this week working on it-- but I'm optimistic!
