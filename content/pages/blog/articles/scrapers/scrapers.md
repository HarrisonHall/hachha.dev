This article contains various thoughts on web scrapers. It doesn't offer any
particular opinion but simply comments on the state of affairs.

## Scrapers

I have nothing against web scrapers. I think there are completely legitimate
uses for scrapers (e.g., search engines). I've had a backlog item for
[slipstream](https://github.com/HarrisonHall/slipstream) to support custom feeds
by parsing webpages for specific selectors.

On the other hand, the current state of crawlers servicing LLMs is ravaging the
internet. As much as reasonably possible, I support the freedom of information
and I understand that all public services are bound to have abusers.

## This site

I added invalid path logging to my [custom site](/blog/site_launch_design) as an
error-checking measure. Every once and a while, I check these logs for the heck
of it. Surprise surprise, a significant portion of these are attacks:

- `.git/config`
- `.ssh/config`
- `system/js/core.js`
- `blog.tar`
- `blog.tar.gz`
- `blog.zip`
- `phpMyAdmin`
- `phpmyadmin`
- `fw.php`
- `database.php`
- `wp-login.php`

> Edit: I'll try to keep this updated over time, if I find anything new. These
> are all from recent logs. Historically, there's been more variance.

My site used to get much more bot traffic, as identified by the error rate on
these logs. Since then, my [robots.txt](/robots.txt) has become significantly
more strict against scrapers. `robots.txt` files are still opt-in, so perhaps it
[isn't the best solution](https://wiki.archiveteam.org/index.php/Robots.txt)
(all-in-all, I don't agree with the linked take). That said, I don't think many
of the proposed alternatives, such as `llms.txt` or the
[Robots Exclusion Protocol (REP)](https://developers.google.com/search/blog/2025/03/robots-future?ref=ppc.land),
are better solutions.

## State-of-the-art

Obviously, anyone can use a firewall. Blocking cloud providers is a good start,
but it can also prevent VPN users from accessing your content.

Last year, Cloudflare pushed out an
[update](https://openrss.org/blog/using-cloudflare-on-your-website-could-be-blocking-rss-users)
that blocked most rss readers from fetching feeds on sites it was protecting. I
avoided this by changing my user-agent. I imagine all bad actors do the same. So
this seems to only hurt the good guys. Cloudflare captcha loops are another
hellhole.

[Anubis](https://anubis.techaro.lol/) seems to be a common choice for admins to
ensure traffic is human-- even the UN uses it! It isn't perfect, proof-of-work
originally meant that humans must be using javascript-enabled browsers, but it
seems to have other scriptless methods. The solution is still a
work-in-progress, the initial release was in early 2025 and there were some
pretty glaring issues (e.g., a week-long cookie). For older and less-powerful
devices, the proof-of-work can take upwards of 10 seconds. And nothing stops
advanced crawlers from simply using a javascript runtime, other than the compute
cost.

## Conclusion

Really, I want the web to stay open, as much as reasonably possible. Maybe if
sites go back to being more lightweight, the weight of crawlers will be less
burdensome?
