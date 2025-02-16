## Inception

While at Anime Frontier last year, I was finally able to play on an authentic
Evangelion pachislot machine. It was fun and visually overwhelming (like many
gambling machines or gambling-inspired games (e.g. Vampire Survivors)) and it
made me want my own!

The idea was simple: a (somewhat-simulated) visual novel that's played via a
pachislot machine. The game automagically plays itself, but by filling up a
meter by "winning at pachislot" you would have the choice between the "red
button," "blue button," and "chance button." Each button would influence the
simulation, moving the character to different areas and triggering semi-random
events.

The game would be implemented in two parts: the "simulation" (backend, rust) and
the "pachinko machine" (frontend, godot). I designed a simple repl to step
through the simulation before a frontend ever got made:

```text
INF::repl: Summary:
           Choice(Chance) Luck(500) Timestamp(Day(0) Hour(4) Step(4))
           Chance: ChanceHistory { len: 1, -1: "Slot(None)" }
           Inventory: CardInventory
           Markers: ProgressMarker("entry"),
           Chars: <Player @par :Luck(0)>, <Blue @par :Luck(0)>, <Red @mar :Luck(0)>, <Mom @mar :Luck(0)>,
           Events:
           Scene: Scene { name: "park_with_blue", desc: "At the park with blue." }
             blue: Sometimes, I wish there were less people at the park.
             blue: But the flowers make up for the noise.
DBG::repl: 1-default 2-chance 3-blue 4-red
```

Each step of the simulation goes through an hour of the 12-hour day and usually
sees the player move from location to location across a 9-tile grid according to
some set schedules, depending on the day. Pressing the "blue" button would move
you towards the "blue" character, while pressing the "red" button would move you
towards the "red" player. Depending on the location, characters, and other game
states, various scenes could occur. Scenes were all defined in a toml file that
would link to various dialogues and effects:

```toml
[[scenes]]
name = "park_with_blue"
description = "At the park with blue."
dialogues = ["park_with_blue_1", "park_with_blue_2"]
locations = ["par"]
whitelist = [
  { characters = ["blue"] },
]
blacklist = []
effects = [
  { luck = 500 },
]
weight = 1
```

Serde is really powerful! It was a breeze handling configuration in various toml
files while defining schema in basic structs.

```rust
/// Character definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterInfo {
    /// Character name.
    character: Character,
    /// Character description.
    description: String,
    /// Possible schedules for character.
    schedules: Schedules,
}

/// Schedules for a character.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedules {
    weekdays: Vec<Arc<DaySchedule>>,
    weekends: Vec<Arc<DaySchedule>>,
}

/// Schedule for a day.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DaySchedule {
    locations: [Location; HOURS_IN_DAY],
}
```

The simulation struct held all possible scenes and just need to point towards
dialogues when asked:

```rust
/// Game scene.
#[derive(Clone, Serialize, Deserialize)]
pub struct Scene {
    name: String,
    description: String,
    dialogues: Vec<String>,
    locations: Vec<Location>,
    whitelist: Vec<SceneCondition>,
    blacklist: Vec<SceneCondition>,
    effects: Vec<SceneEffect>,
    chance: Option<Vec<ChanceEffect>>,
    #[serde(default)]
    weight: SceneWeight,
}

/// Condition to trigger a scene.
#[derive(Clone, Serialize, Deserialize)]
pub enum SceneCondition {
    /// Characters that must **all** be present.
    #[serde(alias = "characters")]
    Characters(Vec<Character>),
    /// Markers that must **all** be present.
    #[serde(alias = "markers")]
    Markers(Vec<ProgressMarker>),
    /// Minimum luck.
    #[serde(alias = "min_luck")]
    MinimumLuck(Luck),
    /// Maximum luck.
    #[serde(alias = "max_luck")]
    MaximumLuck(Luck),
    /// Minimum character rapport.
    #[serde(alias = "min_rapport")]
    MinimumRapport { character: Character, rapport: Luck },
    /// Maximum character rapport.
    #[serde(alias = "max_rapport")]
    MaximumRapport { character: Character, rapport: Luck },
}
```

This was the first major pain point: **I would have to write hundreds of scenes
_and_ thousands of lines of dialogue...** _Yikes_. At least for the gameplay to
be somewhat interesting. The story I was planning was just knockoff Evangelion,
and while I loved the story of Evangelion, I didn't really have any interesting
take on it nor did I really want to write a lot about it.

## Mechanics

In a sense, the game is a rouge-like visual novel. Certain monster attacks (out
of scope of the final game) would cause the run to end. The player would have to
put the events in motion to become a pilot and fight the monster to continue
through the game.

There is a lot to the current game state. Characters move around the city,
following their own little patterns, and each interaction with the player
increases their rapport (measures in "luck") with them. The player can
marginally influence their own movement, but the real fun comes in with
"chance." "Chance" can do anything (although, certain scenes "chanced" would
have specific outcomes), but really there were a few main effects: increase
player score (also measured in luck), increase rapport with a specific
character, gain a progress marker (arbitrary state flag used in other checks),
or get a card (specific or random).

Cards are a lot like inventory. In the final product, I imagined a pachislot
machine that would deal cards out that could be played anytime (much like other
gambling games I've seen in places like ROUND1). Playing a card at the right
scene could have a special effect, either something random or specific.

All state is really tracked through the progress markers. If a scene requires
specific progress markers under certain circumstances, and certain scenes can
give specific progress markers under certain circumstances, really logic is
possible. And while scenes can capture 90% of the actual gameplay, events (a
similar but distinct concept) could be used for tracking longer running "sets of
scenes" the needed to be connected through other means.

## Final Product

I functionally completed the repl. While the final product certainly isn't in a
"fun" state, I can certainly say that I have tested the idea. As-is, it is _not_
fun. I don't think this is because of the simulation itself, but rather the
uninspired writing.

I'm moving on to other projects, but I'm planning on sticking to the simulation
space for now. While this idea didn't go anywhere, with another story (and
perhaps a writing buddy), I'll be down to re-open it.
