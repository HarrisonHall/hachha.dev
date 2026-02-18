There are a lot of patterns I reuse in my personal tools/libraries (e.g.,
logging, cli & config parsing, io, error-handling). I've thought about
abstracting them out into a single helper library, but I believe convenient
reference snippets would be more appropriate.

> [!NOTE]  
> I'm using templates and snippets interchangeably. Personally, I associate
> snippets as being more "finalized" where templates may take parameters and
> modify themselves. :man_shrugging:

I believe that there is a large gap in developer tooling with respect to code
templates. As far as I can tell, most people seem to rely on StackExchange,
Github gists/Gitlab snippets, AI, and past projects for referencing common
patterns in new projects. There does not seem to be any standard way for
searching templates in an organized fashion.

I've tried to look into the prior art for this, but nothing _really_ fits my
requirements.

- The
  [LSP spec](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#snippet_syntax)
  has support for snippets
- There are generic language servers that implement this in various forms:
  - [simple-completion-language-server](https://github.com/estin/simple-completion-language-server)
  - [snippets](https://github.com/lamg/snippets)
- Some IDEs have this as a built-in feature (e.g.,
  [vscode](https://code.visualstudio.com/docs/editing/userdefinedsnippets))
- [Rust-analyzer](ghttps://rust-analyzer.github.io/book/features.html#user-snippet-completions)
  has its own custom snippet system

The feature list I'm looking for is really, in order of priority:

1. Organized, queryable examples that can be fetched within editors
2. Ability to add authorities for snippets
3. Language grammar/context support

## 1. Editor support

I would like to be able to search, from within my editor, something along the
lines of "google-style typescript react function component" and have the snippet
inserted. If some of those terms were derived from context, that would be even
better (3).

## 2. Authorities

I think languages and organizations should be able to publish their templates. I
would like to be able to get templates from specific style-guides (e.g.,
[cppguide](https://google.github.io/styleguide/cppguide.html)), official
language authorities (e.g., [rust-lang](https://rust-lang.org/)), and
professional organizations. The tool should be able to reach out and update the
local snippets, accordingly. Obviously, user-defined snippets should be
supported.

## 3. Language support

Some amount of context should be given. It would not make sense for my python
snippets to show up in rust code.

> [!NOTE]  
> If there were "grammar-agnostic" snippets, that could be a different story.
> But I'm not advocating that inserting search algorithms via a snippet system
> is a good idea.

Many LSPs already support code actions that eliminate some of the verbosity
(e.g., rust-analyzer can create an `impl` block for a `struct`). It would be
nice if this generic tool, instead of adding a block like:

```rust
impl MyStruct {
  fn new() -> Self {
    todo!()
  }
}
```

could tell that I was working on struct `Socket` and had an `impl Error` within
scope:

```rust
impl Socket {
  fn new() -> Result<Self> {
    todo!()
  }
}
```

---

I'm impressed with how far language servers have come. I think a focused effort
on templating/snippet workflows has been under-prioritized and would go much
further than people may give credit.
