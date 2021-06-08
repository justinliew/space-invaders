# Space Invaders @ Edge

This is an implementation of Space Invaders, running as a WASM app in browser.

The idea is that we can also run parts of the WASM app on Compute@Edge to do verification for high scores.

We could also run parts of it on C@E for a potential multiplayer version.

## TODO

- [X] basic player movement.
- [X] sizing: world space, screen space.
- [X] player firing and bullets.
- [ ] enemy firing.
- [ ] game state, losing, lives.
- [X] proper enemy movement, less smooth.
- [ ] enemy speed progression.
- [ ] enemy sprites.
- [ ] explosions.
- [ ] encapsulate relevant game state for picture-in-picture for multiplayer.
- [ ] encapsulate the sim for verification on C@E.
- [ ] network story.
- [ ] ECS (entity-component-system) story for better design/reuse/extensibility.
