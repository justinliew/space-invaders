# Space Invaders @ Edge

This is an implementation of Space Invaders, running as a WASM app in browser.

The idea is that we can also run parts of the WASM app on Compute@Edge to do verification for high scores.

We could also run parts of it on C@E for a potential multiplayer version.

## TODO

### MVP

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
- [ ] swarm needs to move the entire way across based on which ones are still alive.
- [ ] enemy bullets need to spawn from the proper spot, and don't spawn if there's nothing in that row.
- [ ] enemy explosions should be at the center of the enemy.
- [ ] crash, appears to be when some columns are empty. Bullets maybe?
- [ ] do a size pass on everything.
- [ ] mystery UFO.
- [ ] scaling when a window is smaller than the game area is problematic.
- [ ] audio.
- [ ] when I bring up the title bar, the enemies are rendered with a white box.
- [ ] longer bullet delay for player?
- [ ] entering high score name.

### Major Issues/Features

- [X] basic player movement.
- [X] sizing: world space, screen space.
- [X] player firing and bullets.
- [X] proper enemy movement, less smooth.
- [X] enemy firing.

### Architecture

- [ ] proper ECS type structures for more generic collisions, movement, etc.
- [ ] encapsulate relevant game state for picture-in-picture for multiplayer.
- [ ] encapsulate the sim for verification on C@E.
- [ ] network story.
- [ ] ECS (entity-component-system) story for better design/reuse/extensibility.

### Code Cleanup
- [ ] f64 vs usize for point, vector, size.
- [ ] proper functions instead of data.state.world.world_size.x for example.
- [ ] bottom right calculation for swarm is incorrect

### Polish

- [ ] enemy speed progression.
- [ ] do we need to do a better shield damaged texture?
- [ ] ensure shields resize properly