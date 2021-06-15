# Space Invaders @ Edge

This is an implementation of Space Invaders, running as a WASM app in browser.

The idea is that we can also run parts of the WASM app on Compute@Edge to do verification for high scores.

We could also run parts of it on C@E for a potential multiplayer version.

## TODO

### MVP

- [X] player lives. We need a soft reset that resets the player.
- [X] scoring.
- [X] enemy sprites.
- [ ] ensure bullet collisions are exact.
- [ ] shields.
- [ ] add all sprites.
- [ ] swarm needs to move the entire way across based on which ones are still alive.
- [ ] end game needs some sort of delay before restart.

### Major Issues/Features

- [X] basic player movement.
- [X] sizing: world space, screen space.
- [X] player firing and bullets.
- [X] proper enemy movement, less smooth.
- [X] enemy firing.
- [ ] explosions.
- [ ] mystery UFO.
- [ ] entering high score name.
- [ ] audio.
- [ ] scaling when a window is smaller than the game area is problematic.

### Architecture

- [ ] proper ECS type structures for more generic collisions, movement, etc.
- [ ] encapsulate relevant game state for picture-in-picture for multiplayer.
- [ ] encapsulate the sim for verification on C@E.
- [ ] network story.
- [ ] ECS (entity-component-system) story for better design/reuse/extensibility.

### Code Cleanup
- [ ] f64 vs usize for point, vector, size.
- [ ] proper functions instead of data.state.world.world_size.x for example.

### Polish

- [ ] enemy speed progression.