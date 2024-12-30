I've finally gotten around to updating my
[dotfiles](github.com/harrisonhall/dotfiles) after moving over to nixos/wayland.
Despite being a dwm man, I've settled on sway as my wayland compositor of
choice. I never got around to exploring river, but after trying out sway and
seeing how everything _just worked_, I've decided not to break anything.

I've been going through a period recently where I've been trying to update my
toolset to be more modern (in general, this just means I've been replacing tools
with rust equivalents). While there are a lot of things I like about emacs,
speed and size are _not_ two of them.

> Friendship ended with `emacs`, now `hx` is my best friend

`hx` is great. My insert-mode bindings are still very emacs, but I've gotten
pretty used to the modal editing and really enjoy how well it works with minimal
setup.

...

In order to learn more about nix, I decided to try and package something myself.
I used to have a shell alias called `cdtest`, roughly aliased to
`mkdir -p /tmp/test && cd /tmp/test`. This works great to quickly entering into
a temporary directory and trying out some concept without polluting my
workspace. However, I had gotten into a habit of doing _actual_ work in this
directory, which risks hours of work in a blackout. I decided to try my hand at
turning `cdtest` into an
[actual project](https://github.com/harrisonhall/cdtest).

The new `cdtest` is a simple unix-compatible rust program that creates a
temporary project directory in `/var/tmp/cdtest` (and/or `/tmp/cdtest`) and open
a subshell in that directory for work.

Projects are pretty simple, as you can see with their struct definition:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    /// Name of project directory
    pub name: String,
    /// Last cdtest project access time
    #[serde(with = "humantime_serde")]
    pub timestamp: time::SystemTime,
    /// Override already-set configuration
    #[serde(skip)]
    pub force_override: bool,
    /// Duration
    #[serde(with = "humantime_serde")]
    pub garbage_collection: time::Duration,
    /// Whether or directory is temp-only
    #[serde(skip)]
    pub tmp_only: bool,
    /// Whether or not this project already exists
    #[serde(skip)]
    pub existing: bool,
}
```

Running `cdtest foobar` will:

1. Spawn a subshell (using `$SHELL`) in `/var/tmp/cdtest/foobar`
2. Run garbage collection on all projects located in `/var/tmp/cdtest` and
3. Create `/var/tmp/cdtest/foobar/.cdtest.toml` with the following data:

```toml
name = "foobar"
timestamp = "2023-07-07T23:36:03.138895599Z"
garbage_collection = "14days"
```

...
