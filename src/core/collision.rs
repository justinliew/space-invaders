use crate::{game::{Game,GameEvent, ColourIndex}};

impl Game {

	pub unsafe fn handle_collisions(&mut self) {
		let world = &mut self.world;
		let (player, player_bullet, bullets) = world.get_for_collisions();

		// TODO move this to a game event?
		let mut queued_events = vec![];

		bullets.retain(|bullet| {

			let _playerhit = player.check_hit(bullet);
			true
		});

		// TODO
		// if player_bullet.active {
		// }

		for e in queued_events {
			self.send_game_event(e);
		}
	}
}