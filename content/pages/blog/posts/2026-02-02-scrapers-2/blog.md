This is a quick update to my previous [scrapers blog](/blog/scrapers) which I
have recently updated.

Recently I've added additional logging to 404 hits, showing the
`X-Forwarded-For` header value. `X-Forwarded-For` is set by
[Caddy](https://caddyserver.com/), with the client and proxy IP addresses of the
connecting client. I haven't seen anything too surprising there, mostly the
usual suspects (Delhi, Hong Kong) with smattering of assorted European IPs. What
did shock me was seeing my city of residence, Greenville, SC.

Does Greenville host a VPN service I'm unaware of? Further investigation showed
the originator IPv6 being suspiciously similar to my home IPv6. Has my home
server been hacked? If my web server is being attacked by my home server, surely
this must be targeted? I searched my home server to find anything suspicious.
Perhaps I configured a service incorrectly, and it is periodically hitting my
web server for information that doesn't exist?

What I ultimately realized is that [hocko.tech](https://hocko.tech) was set to
reverse proxy [hachha.dev](https://hachha.dev). So all the traffic coming from
Greenville was really coming from my own home and represented attacks on my home
server. I updated Caddy to instead redirect, so I can relax. 😌
