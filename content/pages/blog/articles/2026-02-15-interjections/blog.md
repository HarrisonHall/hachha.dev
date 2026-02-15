I have a bad habit of including thoughts within thoughts (like this!) when
writing.

> [!TIP]  
> GFM markdown has a great way to handle this, alerts!

So I've added them to my site.

To support this, I've switched from `markdown-rs` to `comrak`, which has some
additional features that are quite nice.

In addition to the usual suspects,

> [!NOTE]  
> The usual suspects are **bold** and _italics_.

[[hachha.dev|/]] now supports ==highlights==, sub~sub~ and super^super^ scripts,
CJK-friendly emphasis **このように**, footnotes[^footnote] & inline
footnotes^[an inline footnote!], and ||spoilers||.

[^footnote]: An example footnote.
