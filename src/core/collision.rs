// TODO these aren't core game functions
use crate::leaderboard::{prep_leaderboard_entries};

use crate::bullet::BulletType;

use crate::game::{Game};

type DeferredShieldDamage = (usize,usize,usize);

pub unsafe fn handle_collisions(state: &mut Game) -> Vec<DeferredShieldDamage> {
	let world = &mut state.world;
	let player = &mut world.player;
	let swarm = &mut world.swarm;
	let bullets = &mut world.bullets;
//	let particles = &mut world.particles;
	let shields = &world.shields;
	let ufo = &mut world.ufo;

	let mut deferred_shield_damage = vec![];

	bullets.retain(|bullet| {

		// shields first
		for (index,shield) in shields.iter().enumerate() {
			match shield.check_hit(bullet) {
				Some((i,j)) => {
					deferred_shield_damage.push((index,i,j));
					return false;
				},
				None => {},
			}
		}

		let playerhit = player.check_hit(bullet);
		if playerhit {
			// TODO
//			make_explosion(particles, &Point::new(bullet.x(), bullet.y()), 8, ColourIndex::RED);
		}
		!playerhit
	});

	if let BulletType::Player(alive) = world.player_bullet.bullet_type {
		if alive {
			// shields first
			for (index,shield) in shields.iter().enumerate() {
				match shield.check_hit(&world.player_bullet) {
					Some((i,j)) => {
						deferred_shield_damage.push((index,i,j));
						world.player_bullet.bullet_type = BulletType::Player(false);
						break;
					},
					None => {},
				}
			}

			let swarmhit = swarm.check_hit(&world.player_bullet);
			if let Some(hit) = swarmhit {
				let points = hit.0;
				let loc = hit.1;
				// TODO
//				make_explosion(particles, &Point::new(loc.x,loc.y), 5, ColourIndex::WHITE);
				state.score += points as i32;
				prep_leaderboard_entries(&mut state.leaderboard, "Justin", state.score);
				// TODO
				// push_leaderboard_entries(&state.leaderboard);
				world.player_bullet.bullet_type = BulletType::Player(false);
			}

			let ufohit = ufo.check_hit(&world.player_bullet);
			if let Some(hit) = ufohit {
				let points = hit.0;
				let loc = hit.1;
//				make_explosion(particles, &Point::new(loc.x,loc.y), 5, ColourIndex::BLUE);
				state.score += points as i32;
				prep_leaderboard_entries(&mut state.leaderboard, "Justin", state.score);
				// TODO
				// push_leaderboard_entries(&state.leaderboard);
				world.player_bullet.bullet_type = BulletType::Player(false);
			}
		}
	}
	deferred_shield_damage
}