After the height of Vampire Survivors popularity, I've wanted to make a clone.
After workshopping the idea in my head for half a year, I settled on an idea:
"Vampire Survivors but you manage the world like Stardew Valley with combat
strategy like Plants Vs. Zombies." There's some more nuance to the idea, but I
was quite satisfied with it conceptually. Time to get to work!

There are _soft_ requirements I had for this game idea that I won't state. I
tend to find motivation/value in these projects based on my subjective criteria
(e.g. challenge and (relative) novelty). Since the game _had_ to have these
factors, the approach I took during development was similar each time: 1) player
movement, 2) enemy boid movement, 3) interactions, 4) physics, 5) networking. As
this idea evolved over my attempts to bring it to life, many aspects of the
design changed, introducing new soft requirements.

## Attempts

### Attempt 1

I am _very_ familiar with Godot, so of course my attempt at an MVP would start
here. It was straightforward to add some scenes for the player, an enemy, and a
couple of flowers. Throw in [Nakama](https://github.com/heroiclabs/nakama) for
networking, and I could try it with some friends.

For me to find "value" (really, motivation) in this work, I need to be
challenged and I need the product to be "perfect." One of my hard requirements
was that the game _had_ to run on my laptop, not just my desktop. So how did it
work on a laptop? Not great.

![Oh the jitter!](/blog/a_year_on_game_engines/attempt1.gif)

Aside from some of the replication kinks (e.g. jittering) that needed to be
worked out, I could only spawn ~300 enemies at a time! Surprisingly enough,
performance was only marginally better on my desktop! What gives? Now there are
plenty of ways to optimize in Godot (here, using the physics and rendering
servers), but I had another idea. I _wanted_ that high of _knowing_ my software
was the fastest it could be, so I tried again another way.

### Attempt 2

Rust is far from a perfect language, but the peace-of-mind the type system
offers is incredibly pleasant. Any time I have the opportunity to grow my skills
in the language, I try my hand. Bevy is one of the cool-kids-on-the-block wrt
Rust, and with a friend preaching to me about it since College, I had to give it
my go.

ECS is surprisingly different from OO game design. Even if you use composition
in your "OO" engines, the way you have to reason about ECS interactions takes a
while to wrap your head around. This was a fun exploratory project, but I
enjoyed the niceties Godot offered too much, so I put the idea down for a while.

### Attempt 3

Godot has an incredible interface for designing scenes and writing scripts. Bevy
is great for simulating systems. Why not combine the two? The architecture was
well-thought-out. A Godot game would initialize a rust extension (using
[gdext](https://github.com/godot-rust/gdext)) which would handle the actual game
simulation. The extension would start a tokio runtime for responding to queries
from Godot about the current state of the system and handle inputs and network
events. The extension would also handle bevy execution and pass messages via
async queues. Surprisingly, it worked-- well, even!

![3d boids!](/blog/a_year_on_game_engines/attempt3.gif)

At one point, I counted 56 threads being used in the game process! It _felt_
wasteful, but I couldn't complain about performance. The "model" and "view"
could be edited completely independently, if a model wasn't defined in the view,
a giant "?" would display in its place. So why wasn't this the final attempt?
Uncertainty. Up until this point, I hadn't thought much about the map. Would it
be hand-designed or randomly generated? How should I pass map data between the
model and the view? Marching cubes? BSP files? How is physics handled? Rapier
lacked the features I needed and XPBD (Avian) wasn't sufficiently mature. I
started writing my own basic physics engine myself (which had gotten much
further than _I_ would have guessed), but when fun became work I put the idea
down for a while.

### Attempt 4

Zig is surprisingly pleasant to work in. The language itself is simple, it
builds fast, and it has incredibly easy c interop. There's something so
satisfying about making an incredibly performant program, so why not make a
custom framework this way? I could take what I liked from Godot, what I liked
from other engines (a scene tree from godot, simplified entity handling with
some ideas from the Mario 64 decomp, etc.).

![Voxels 1!](/blog/a_year_on_game_engines/attempt4_voxels.png)

![Voxels 2!](/blog/a_year_on_game_engines/attempt4_final.png)

From the screenshots, you should be able to tell that the primary focus of this
rewrite was going to be getting the map done **first**. I chose a simple voxel
solution, with an in-game editor, to handle world building. Everything was
rendered via [raylib](https://github.com/raysan5/raylib) (surprisingly, while I
enjoy performance programming/program optimization, I don't currently enjoy raw
graphics programming...).

So what was the nitpick that made me abandon this version? Nothing, really.
After working on an idea for a long time, some of the itch is scratched and the
idea becomes tiring. You leave for vacation for a week and the project doesn't
get picked back up.

## The Future

So Godot is slow and doesn't make you "feel" like you're in complete control of
the executable. Bevy is performant but is difficult to develop in. Custom
frameworks give you complete control but once you've made progress you begin to
wish for the niceties back. It's a never-ending cycle!

This post is a _very_ high overview over some of the ways I explored making this
game idea. Honestly, I've forgotten many of the specifics over the past couple
of years. The journey was fun, even if there wasn't a final end product (not so
different from many of my side projects).

I am no longer pursuing this idea. There's a new shiny game idea I want to make,
so I'm going to continue to work on that. Many of my older game projects make it
onto [github](https://github.com/trackl-games), but most are still private.
Exploring prototypes with different engines, frameworks, and techniques is far
from a waste of time-- sure I haven't actually accomplished the original "goal,"
but I've learned a lot and developed my creative outlet. As for how I'll work on
this new idea, I still haven't learned my lesson-- _hopefully_ attempt 3 will be
my last.

## Notes

- I've learned I should document my side projects much more thoroughly, it was
  difficult to find development videos!
- I have since reworked part of the "Attempt 4" framework as part of another
  project, this time including flecs for some simple ecs.
- While Nakama was great with a prototype, I've been working on my own
  multiplayer framework in rust for a while. It's far from ready, but that's
  what I've been plugging in the other attempts or projects where networking was
  explicitly mentioned.
