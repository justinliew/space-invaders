# Space Invaders @ Edge

This is an implementation of Space Invaders, running as a WASM app in browser.

The idea is that we can also run parts of the WASM app on Compute@Edge to do verification for high scores.

We could also run parts of it on C@E for a potential multiplayer version.

## TODO

See Github Issues for tracking.

### For Release

- [ ] if you kill the UFO you can keep shooting the area and get points.
- [X] ranks for leaderboard.
- [X] name the 2 leaderboards.
- [X] entering high score name.
- [X] after dying once the wave gets set to 0, not 1. I fixed this but it doesn't seem to work.
- [X] don't show me in the global leaderboard if I'm not in rank.
- [X] dedup local player.
- [X] get global and around me if relevant.
- [X] aroundme leaderboard issue when we move up the - [X] show local score differently.
- [X] proper POST of score.
- [X] hook up proper leaderboard.
- [X] fix crash.
- [ ] implement redis for leaderboard.

### Architecture

- [ ] encapsulate the sim for verification on C@E.

### Architecture

- [ ] we need a way to save and load state.
- [ ] we need a way to verify/checksum the state.
- [ ] we need to keep a frame number/count.
- [ ] we need to figure out how to encapsulate input.
- [ ] encapsulate relevant game state for picture-in-picture for multiplayer.
- [ ] do we need to render to framebuffer so we can easily do that on C@E?
- [ ] network story.
- [ ] proper ECS type structures for more generic collisions, movement, etc.
- [ ] ECS (entity-component-system) story for better design/reuse/extensibility.

### Code Cleanup

- [ ] proper functions instead of data.game.world.world_size.x for example.
- [ ] bottom right calculation for swarm is incorrect
- [ ] inputs should have pressed/release plus "changed". This will help our UI transitions.

### Polish

- [ ] audio.
- [ ] better title screen with proper title.
- [ ] scaling when a window is smaller than the game area is problematic.
- [ ] lives gained when hitting score thresholds.
- [ ] enemy speed progression.
- [ ] do we need to do a better shield damaged texture?
- [ ] ensure shields resize properly if the screen size changes
- [ ] player sizing, radius for collision, etc. Should the player be bigger?

### Done

- [X] f64 vs usize for point, vector, size.
- [X] some sort of event system for emitting events from game.
- [X] explosions.
- [X] leaderboards updating after score update.
- [X] after the separation refactor, explosions seem slower. Is this worth addressing?
- [X] stub in leaderboard GET functions.
- [X] win state and transition to new
- [X] UX overhaul. draw leaderboard.
- [X] draw title menu.
- [X] mystery UFO.
- [X] player lives. We need a soft reset that resets the player.
- [X] scoring.
- [X] enemy sprites.
- [X] ensure bullet collisions are exact.
- [X] explosions.
- [X] add all sprites.
- [X] shield collision.
- [X] death needs more of a delay too.
- [X] end game needs some sort of delay before restart.
- [X] shields need to reset on game over.
- [X] swarm needs to move the entire way across based on which ones are still alive.
- [X] enemy bullets need to spawn from the proper spot, and don't spawn if there's nothing in that row.
- [X] enemy explosions should be at the center of the enemy.
- [X] enemy bullet spawn times should change as there are fewer enemies.
- [X] crash, appears to be when some columns are empty. Bullets maybe?
- [X] do a size pass on everything.
- [X] when I bring up the title bar, the enemies are rendered with a white box.
- [X] basic player movement.
- [X] sizing: world space, screen space.
- [X] player firing and bullets.
- [X] proper enemy movement, less smooth.
- [X] enemy firing.
- [X] longer bullet delay for player
- [X] f64 vs usize for point, vector, size.
- [X] some sort of event system for emitting events from game.
- [X] explosions.
- [X] leaderboards updating after score update.
