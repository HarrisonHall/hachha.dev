I have moved [git.hachha.dev](https://git.hachha.dev) from
[gitea](https://about.gitea.com/) to [radicle](https://radicle.xyz/). I've been
following radicle for a couple of years now, but I figured it was finally time
to actually follow through and thoroughly investigate it.

At its core, radicle is a decentralized git forge. So github, but without
microsoft. Since it isn't centralized, the risk of having useful software taken
down by overeager legal teams should be far lower. Radicle still supports
private repos. It uses a custom P2P protocol based off of Secure Scuttlebutt
(SSB).

## Setting up radicle

Radicle has a pretty detailed official guide online, but I think it is useful to
document the workflows I use. After installing radicle, run `rad auth` to create
your local identity. `rad node start` is used to manually start the background
daemon. The background daemon

## The seed node

My home server is my seed node.

```
sudo groupadd --system seed
sudo useradd --system --gid seed --create-home seed
```

## TODO

- Set up DNS-based discovery

> [!NOTE]  
> I'm using templates and snippets interchangeably. Personally, I associate
> snippets as being more "finalized" where templates may take parameters and
> modify themselves. :man_shrugging:
