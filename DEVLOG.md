## Development Log For [My First Game Jam](https://itch.io/jam/my-first-game-jam-winter-2020)  

I'm building a curling game using Rust and a framework that I have little-to-no experience with. I have 2 weeks to finish. Let's learn as we go.

Here are my daily logs:

* [Day 1 - 1/28](https://github.com/mschmo/amethyst_curling/blob/master/DEVLOG.md#day-1-2020-01-28)
* [Day 2 - 1/29](https://github.com/mschmo/amethyst_curling/blob/master/DEVLOG.md#day-2-2020-01-29)
* [Day 3 - 1/31](https://github.com/mschmo/amethyst_curling/blob/master/DEVLOG.md#day-3-2020-01-31)
* [Day 4 - 2/03](https://github.com/mschmo/amethyst_curling/blob/master/DEVLOG.md#day-4-2020-02-03)
* [Day 5 - 2/06](https://github.com/mschmo/amethyst_curling/blob/master/DEVLOG.md#day-5-2020-02-06)
* [Day 6 - 2/08](https://github.com/mschmo/amethyst_curling/blob/master/DEVLOG.md#day-6-2020-02-08)

---
### Day 7 [2020]

I had fun with this project, but I don't feel I've had enough time to accomplish everything I wanted during the jam. So I am going to continue working on this game for personal enjoyment and learning.

I want to get collision working. That would be satisfying.

The collision is said to be "elastic". References:
1. http://www.dynamicscience.com.au/tester/solutions1/flight/winterolympics/curln.html
2. https://isaacphysics.org/concepts/cp_collisions
3. https://en.wikipedia.org/wiki/Elastic_collision

```
If both masses are the same (which we will have for now), we have a trivial solution:
v1 = u2
v2 = u1
```

And cool, collisions actually appear to be working:

![Elastic collisions](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_7_elastic_col.gif)

Fair bit of cleanup. Using `Entities<'s>` to store unique id for collision comparison.

### Day 6 [2020-02-08]

* UI Text
* Actually rotate turns
* Export (macOS) and upload to itch.io

![final project, but still wip...](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_6_final.gif)

`$ cargo build --release`

`Finished release [optimized] target(s) in 8m 11s`

### Day 5 [2020-02-06]

Couple days off. Watching Khan Academy videos on basic geometry, trig, calc and physics.

TODO:
* Take turns
* Collision detection
* Added a `StoneState`

![detecting collisions](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_5_col_detection.gif)

https://rust-gamedev.github.io/posts/survey-01/

### Day 4 [2020-02-03]

I didn't do any work on this project in the past couple days. Instead I went snowboarding out in this frozen tundra of Winter Park, CO:

[pic?]

But we're back in action now. And what did we accomplish today?

link of the day: https://www.gamedev.net/forums/topic/603877-can-somebody-explain-velocity-and-acceleration-in-laymans-terms/
Things that I definitely sat through classes on, but now I'm actually excited to learn once I found a fun application for them in the real-word.
The concept of a vector was lost on me in school. 

* debug line expands to reflect power of shot (DONE)
    * coordinate chart showing how the formula was figured out
* fixing the angle of the shot (DONE)
    * It's amazing how much of calculus from college I have already forgotten.
    * https://gamedev.stackexchange.com/questions/25277/how-to-calculate-shot-angle-and-velocity-to-hit-a-moving-target
    * https://math.stackexchange.com/questions/707673/find-angle-in-degrees-from-one-point-to-another-in-2d-space
    * What's preferred, degrees or radians?
    * https://en.wikipedia.org/wiki/Atan2
    * well holy shit, atan2 for the angle and cos/sin on the angle for the velocity was the answer... but what does it mean?
    
We can use `Atan2(y_mouse - y_stone, x_mouse - x_stone)` to get the angle between the ray from the stone to the mouse and the positive x-axis.

```rust
for (stone, transform) in (&mut stones, &transforms).join() {
    let a = (end_world.coords.y - transform.translation().y).atan2(end_world.coords.x - transform.translation().x);
    stone.velocity[0] = a.cos() * self.launch_velocity;
    stone.velocity[1] = a.sin() * self.launch_velocity;
}
```

### Day 3 [2020-01-31]  
  
*This log is a WIP. I've been working fast and loose in the past 24 hours, and haven't had a time to organize and edit these thoughts, so ATM this is not very readable. Just a random/incoherent dump of my thoughts from the day which I will try to clean up and put in a more presentable format.*

Good ol' Entity Component System. We've added some entities (e.g. `Stone`) and components (e.g. `Transform`). Today, we're going to add our first system to the game (possibly multiple, I have no idea how this day is gonna go yet). Systems are objects that represent operations over entities. So I'm thinking we'll have a system to move our stones, as well as one to detect for collisions between them.

We're also going to add a system that will move a curling stone in the direction of a mouse click. Crap I might need to skip to chapter 4.4 for this, as the stone movement will be much more similar to that of the ball in pong. I'm gonna read through them both first and then determine the which features from both should apply to the curling stones.  
  
Getting the mouse position and checking when the left mouse button is pressed:  
```  
Some((59.273438, 721.9922))  
Some((59.273438, 721.9922))  
Some((44.875, 740.5508))  
Some((44.875, 740.5508))  
Some((39.273438, 744.91406))  
Some((39.273438, 744.91406))  
```
But there's a major problem. Our mouse position has coordinates for a completely different plane than our stones do.

I didn't realize this at first. I was getting some super funky results when attempting to compare a stone's position to the mouse cursor. The stone is sitting on a plane whose origin (0, 0) is sitting in the middle of the camera. Our mouse's origin however is at the top left of the screen. *TODO: Verify what I just said and provide a diagram that shows the two coordinate planes on top of each other*
*More TODO: Show investigation of how we solved this. 1) Searching on discord 2) Looking at the mouse_raytrace.rs example 3) Any tweaks I made from the example to adapt to our game*

I found a question posed by someone on discord who is trying to accomplish the same thing as me:

![discord help](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_3_discord_help.png)


It was at this point that I just started going wild, making changes left and right. Adding velocity here. Adding acceleration there. Mutating every entity and component I can get my hands on. And committing absolutely nothing with git! Whatever happens happens!
  
Here's a basic question. How do things move? This looks like it could be useful:

```rust
/// Move relatively to its current position and orientation.
///  
/// For example, if the object is rotated 45 degrees about its Y axis,
/// then you append a translation along the Z axis, that Z axis is now
/// rotated 45 degrees, and so the appended translation will go along that
/// rotated Z axis.
///
/// Equivalent to rotating the translation by the transform's current
/// rotation before applying.
#[inline]
pub fn append_translation(&mut self, translation: Vector3<f32>) -> &mut Self {
    self.isometry.translation.vector += self.isometry.rotation * translation;
    self
}
```

One thing I am beginning to notice is compile times. Every time I modify a single line of code, I have to wait another ~30 seconds to test the game. I may ask around if there are any solutions to that, but for now I need to figure out how to best spend those extra 30 seconds.

Welp, this is **disgusting**, but it works well enough:

```rust
for (stone, transform) in (&mut stones, &transforms).join() {
  let x = match (end_world.coords.x - transform.translation().x) > 0.0 {
  true => self.launch_velocity,
  false => -self.launch_velocity
    };
  let y = match (end_world.coords.y - transform.translation().y) > 0.0 {
  true => self.launch_velocity,
  false => -self.launch_velocity
    };
  stone.velocity[0] = x;
  stone.velocity[1] = y;
  println!("Launch {:?} stone at initial velocity = {:?}", stone.color, stone.velocity);
}
``` 

It hasn't clicked yet, but I get a feeling if I keep working at it things will eventually click.

It's following but the angle is strange. It consistently stays at a 1,1 slope.

There was an amethyst release today! Dope, so I probably don't need to be pinning a git git hash anymore, as this latest release should have everything I need.

*TODO: Section on collision detection. 1) Show research/investigation. 2) Evaluating example from pong tutorial (between ball, walls, and paddles) 3) Formula used to detect collision between two circles*

```rust
let s1_s2_distance = ((s1_x - s2_x).powf(2.0) + (s1_y - s2_y).powf(2.0)).sqrt();
match s1_s2_distance {
  d if d <= s1.radius + s2.radius => println!("Ooh they're touching!"),
  _ => println!("Not Touching")  
}
```

So now we at least know when two stones are touching. Of course, we do absolutely nothing about it and allow the two stones to morph together. But we'll add some proper physics another day.
*TODO: show gif of them overlapping with debug message stating the obvious*

Overall, what did I learn about in this lesson?
1. Ray Casting
2. Collision Detection
3. Systems in an ECS architecture

Cool. And what are the next steps? Well, according to the tutorial we're suppose to start keeping score. But I may need to focus more on the core movement mechanics.

*TODO: Talk about how you added a debug line drawing to indicate when a stone is being charged for a launch*

Final product of the day:

![launching stones](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_3_final.gif)

### Day 2 [2020-01-29]

What's the game going to look like? Time to decide.  

We will probably have around three different [states](https://book.amethyst.rs/stable/concepts/state.html) in our curling game. `MainMenuState`, `GamePlayState` and `PauseState`. For now, we will focus mainly on the `GamePlayState` as it is the most critical for a fun game.
  
![rough sketch](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_2_scene_sketch.jpg)   

I've also roughly outlined some of my needs vs wants for this project. Things that I believe are more critical for the MVP vs things that I think would be bad ass to have, but aren't critical for the MVP.
    
Game mechanics for our curling game that will be the same as the pong game:    
* We want the camera to cover the entire arena  
* There are two players
* Collision detection and bouncing  

Mechanics that will differ or are not present in the pong tutorial:
* Over-top layering (stone on top of target) which will be made up of several scoring rings
* Movement will be done by mouse click instead of keyboard. Distance + angle from stone will make up its initial velocity and launch angle
* At first glance I don't see a section in the tutorial where a main and pause menu are added
* Turn-based vs real-time
  
In [this section](https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-02.html) of the tutorial we're going to add a struct for one of the game's main entities, the paddle. But let's switch that with a curling stone. Basically everywhere the pong tutorial uses `Paddle`, I will use `Stone`.  

I drew two curling stones, one with a blue handle and one with a red handle, as well as the target area. The stones are 16x16 pixels and the target is 128x128 pixels. Those sprites now live in this [sprite sheet](https://github.com/mschmo/amethyst_curling/blob/master/assets/texture/curling_spritesheet.png) which I made using the pixel art editor [aesperite](https://www.aseprite.org/).

I followed the tutorial along for a bit to the point where I was ready to test drawing out a curling stone of each color, and display them both in the center of the screen. There wasn't too much code that I needed change from the tutorial - mostly sprite sheet coordinate settings. So let's try it out...

![oh my goodness](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_2_big_error.png) 

Holy crap, that's doesn't seem very good, and our game is still a blank screen. But hey, the game did compile and run... let's keep that in mind while debugging.  
  
So what the heck does this error mean? I see this message:  
```  
Failed to load asset with name "texture/curling_spritesheet.ron"  
```  
As well references to a `Parser` and `ExpectedIdentifier`. Sounds like there is an problem parsing the sprite sheet's RON file. Let's check if there are any Github issues that could be related to this error.  
  
Ok, I came across [this](https://github.com/amethyst/amethyst/issues/2034) issue. **@jbdutton** claimed that removing the `List()` wrapper resolved the error for him. Hmm... but our RON structure is already setup like that. Let's think. That parser error contains this bit of text `Position { col: 1, line: 1 }` which sounds like the parser breaks at the very first char. Also the first char is where `List()` originally was in that issue. Let's try adding that back in just for the hell of it and see what happens, because even if we get a different error message that could be a clue to solving this problem.  
  
And... it works! I'll make a mental reminder to someday investigate this potential bug in the pong tutorial's code, but for right now I'm going to continue working on the game itself.  
  
Because there still is a little issue. The stones are tall and out of position.

![squeezed stones](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_2_squeezed_stones.png)   

After messing around with the values of `ARENA_WIDTH` and `ARENA_HEIGHT` (used for our camera and sprites' transformations), things started looking right. What seems to have fixed things is keeping their `width x height` relation the same as that defined in `config/display.ron` - I divided those dimensions by 2 in this case. I should definitely sketch out a chart showing the coordinate space used for those transformations instead of just tweaking them until the game looks okay.

![done for the day](https://raw.githubusercontent.com/mschmo/amethyst_curling/master/screenshots/day_2_final.png)

But cool, I feel good leaving it here for the day. Tomorrow we will start to make things move!
  
---

### Day 1 [2020-01-28]  
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
    
Hmm, but it still fails to build.  It looks like it could be this [issue](https://github.com/amethyst/amethyst/issues/2011).  The last comment I see is from 12 days ago.  **@gamecubate** says it works on master so let's try using the most recent commit. I add [this](https://github.com/amethyst/amethyst/commit/1ef6b1a673db8b7c3dd5142905b2f) sucker to my `Cargo.toml` and try again...    
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
