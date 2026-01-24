I like to tinker around with this site, trying different techniques to see how
things work. [hachha.dev](https://hachha.dev) v0.12.0 has made the switch from
[`spectre.css`](https://picturepan2.github.io/spectre/) to
[`pico.css`](https://picocss.com/).

There were a few reasons for the switch:

- I've used `pico.css` in other projects and respect it.
- Limiting the css frameworks I use will ensure I don't have to relearn each one
  each time.
- `pico.css` is mostly classless, simplifying styling.

One of the advantages to using a more-popular css framework is device support.
`pico.css` claims:

> All typographic elements are responsive and scale gracefully across devices
> and viewports.

which should ensure this site remains accessible on various devices. In
practice, I'm not 100% sure I personally like the font changing sizes across
devices, but I imagine there's a reason this exists.

Professionally, I have found [`tailwind`](https://tailwindcss.com/) to be a
great tool for components. But for sites like this where each page is pretty
hand-crafted (and not using Node!), `tailwind` is a bit overkill. What I find
really convenient about tailwind is some of the basic utility classes: `flex`,
`flex-col`, `mx-auto`, `pt-[2em]`, etc. I've virtually recreated the most used
classes in this site's custom `contents.css` file.

What I really want in a minimal semantic css framework is an opinionated set of
the most common tailwind classes with semantic meaning. Instead of parsing and
building css classes from patterns (e.g., `h-[<some-custom-size>]`), provide
`h-sm`, `h-md`, `h-lg`, and other classes similarly. Where things like colors
and sizes are overridable by some obvious css variables.

[`Bulma`](https://bulma.io/) seems to kinda provide this with
[spacing](https://bulma.io/documentation/helpers/spacing-helpers/) and
[flex](https://bulma.io/documentation/helpers/flexbox-helpers/) helpers. I
haven't given it a try yet (I couldn't justify the 664KiB minified css file for
this site, the horror!), but I am heavily considering it for a small project I
am currently working on.
