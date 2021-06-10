# Space Invaders @ Edge

This is an implementation of Space Invaders, running as a WASM app in browser.

The idea is that we can also run parts of the WASM app on Compute@Edge to do verification for high scores.

We could also run parts of it on C@E for a potential multiplayer version.

## TODO


- [X] basic player movement.
- [X] sizing: world space, screen space.
- [X] player firing and bullets.
- [X] proper enemy movement, less smooth.
- [ ] end game needs some sort of delay before restart.
- [ ] swarm needs to move the entire way across based on which ones are still alive.
- [ ] constrain player.
- [ ] enemy firing.
- [ ] player lives.
- [ ] shields.
- [ ] enemy speed progression.
- [ ] enemy sprites.
- [ ] scoring.
- [ ] explosions.
- [ ] mystery UFO.
- [ ] entering high score name.
- [ ] audio.
- [ ] encapsulate relevant game state for picture-in-picture for multiplayer.
- [ ] encapsulate the sim for verification on C@E.
- [ ] network story.
- [ ] ECS (entity-component-system) story for better design/reuse/extensibility.
