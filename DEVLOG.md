
## Development Log For [My First Game Jam](https://itch.io/jam/my-first-game-jam-winter-2020)
  
#### Day 1  
Ok so we're a few days into this jam and I haven't started yet. It took me a little while to choose both the type of game I wanted to make as well as with what I wanted to make it with. Overall I want this to be a learning experience, which honestly any game or tool would be since I'm not very familiar with game development. I've mostly copied and studied online tutorials in the past and never truly completed a full game purely on my own. I'm excited to begin.
  
The theme for this jam is "cold". I went through a bunch of different ideas and concepts, such as a game where you are a perishable food item, like an orange or carton of milk, and must maintain a cool temperature in order to survive. But ultimately I wanted to keep the game concept as simple as possible so that I could try a more challenging tool to build it with. I decided on an ice curling game. All right, what are the rules of curling? Good question. I have no idea. But it looks simple. I'm a visual learner, and I found an explanation on [youtube](https://www.youtube.com/watch?v=TjxcZhbVSVQ). But for right now, the rules aren't even important. I have a lot of work to do setting up my environment, designing the main game objects and writing their functionality. Basically the only thing we need to do right now is create a stone that can slide on ice in a particular direction with an initial velocity, and collide with other objects of its type.
  
For the game engine I chose [amethyst](https://amethyst.rs/), a game engine built with rust. Initially the concept of building a game entirely via code in an IDE made me nervous. I thought it could be easy to get lost without the help of a constant view of the game's visual environment that tools like [Unity](https://unity.com/) and [Godot](https://godotengine.org/) provide. Even with HTML5/JS it's quite simple to view the game environment in real-time through many different rendering applications, though with no ability to move around in it.

The good news is I am a little familiar with rust. Though I haven't used rust in a while and would be nice to dust off my knowledge. I think I know enough to get by for this specific project. It will be a challenge, but perfect, that's what I'm looking for.

It's also nice to see how active the amethyst discord community is. I only lurk because I don't know enough on the subject of rust or amethyst to contribute to conversation, but I still enjoy the occasional time I actually understand something that someone said.

Another reason amethyst is appealing is that my day job is in data engineering, and amethyst claims to be a data-driven game engine. While the term "data-oriented design" sounds sweet and useful for my professional career, I really don't know if the concepts I learn from an [ECS pattern](https://en.wikipedia.org/wiki/Entity_component_system) would be able to apply to and [ETL](https://en.wikipedia.org/wiki/Extract,_transform,_load) system. But hey we'll see, never know what we'll find out.  

*Cool, sounds good. Let's get rolling.*

I messed around with amethyst a while ago, but never got very far. So I started with the introduction in the [amethyst book](https://book.amethyst.rs/stable/intro.html). My approach to both learning and building at the same time will be this: follow along in the [pong game tutorial](https://book.amethyst.rs/stable/pong-tutorial.html), and swap out the pong components with curling components where possible as I go along. I figure I can then use that finished project as boilerplate for my game.

For day 1, I will complete [section 1](https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-01.html) of the tutorial. The end result will be displaying a blank game window. I will be developing this on macOS 10.15.3.

So I tried hitting `cargo build` for the first time...  
```  
error: failed to run custom build command for `gfx-backend-metal v0.2.4`.  
```
Search the internet, turns out I need to install xcode. Done.

Hmm, but it still fails to build.  It looks like it could be this [issue](https://github.com/amethyst/amethyst/issues/2011).  The last comment I see is from 12 days ago.  @gamecubate says it works on master so let's try using the most recent commit. I add [this](https://github.com/amethyst/amethyst/commit/1ef6b1a673db8b7c3dd5142905b2f) sucker to my `Cargo.toml` and try again...
```rust
   Compiling amethyst_curling v0.1.0 (/Users/mschmoyer/Projects/amethyst_curling)  
error[E0599]: no method named `with_clear` found for type `std::result::Result<amethyst_rendy::plugins::window::RenderToWindow, amethyst_config::ConfigError>` in the current scope  
  --> src/main.rs:33:22  
   |  
33 |                     .with_clear([1.0, 1.0, 1.0, 1.0]),  
   |                      ^^^^^^^^^^ method not found in `std::result::Result<amethyst_rendy::plugins::window::RenderToWindow, amethyst_config::ConfigError>`  
  
error: aborting due to previous error  
  
```  

Damn. New error. Seems likely this could be an issue with my version of amethyst not playing nice with the code I copied from the pong tutorial implementation. And the bug occurs in the section where we initialize this `RenderToWindow` guy:
```rust
RenderToWindow::from_config_path(display_config_path) 
  .with_clear([1.0, 1.0, 1.0, 1.0]),
  ```
  I can see that `with_clear()` is still defined on the amethyst version I'm using, so I wonder if the type returned from `from_config_path` has changed. I see `Result` out front... so hey maybe this just needs unwrapped? I add a [? operator](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html) and boom we're golden. The game builds and runs. Love it.
  
Set the background to white so the ice graphics are complete:

![ice ice baby](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_1.png)

Tomorrow night I will work on step 2 of the tutorial. It has us drawing the paddles. Instead I'll be drawing curling stones.
