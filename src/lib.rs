extern crate itertools_num;
use std::os::raw::{c_double, c_int, c_char, c_uint};
use std::sync::Mutex;
use std::f64;

mod input;
mod state;
mod leaderboard;

#[path = "./entities/bullet.rs"]
mod bullet;

#[path = "./entities/player.rs"]
mod player;

#[path = "./entities/swarm.rs"]
mod swarm;

#[path = "./entities/particle.rs"]
mod particle;

#[path = "./entities/ufo.rs"]
mod ufo;

#[path = "./entities/shield.rs"]
mod shield;

#[path = "./core/point.rs"]
mod point;
#[path = "./core/vector.rs"]
mod vector;

#[path = "./core/size.rs"]
mod size;

use crate::swarm::Swarm;
use crate::size::Size;
use crate::bullet::{BulletType};
use crate::state::{State,GameData,GameState, ResetType};
use crate::point::Point;
use crate::vector::Vector;
use crate::particle::{ColourIndex,Particle};
use crate::shield::Shield;
use crate::leaderboard::{get_leaderboard_entries,prep_leaderboard_entries, push_leaderboard_entries};

#[macro_use]
extern crate lazy_static;

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double);
    fn draw_bullet(_: c_double, _: c_double);
	fn draw_player_bullet(_: c_double, _: c_double);
    fn draw_particle(_: c_double, _: c_double, _: c_double, _: c_int);
	fn draw_ufo(_: c_double, _: c_double);
    fn draw_hud(_: c_int, _: c_int, _: c_int);
	fn draw_intro();
	fn draw_game_over(_: c_int);
	// fn draw_debug(_: c_double, _: c_double, _: c_double, _: c_double);
	// fn draw_bounds(_: c_double, _: c_double, _: c_double, _: c_double);

	// id, 5x5
	fn init_shield(_: c_int);
	fn add_shield_state(_: c_int, _: c_int, _: c_int);

	// id, index, state
	fn update_shield(_: c_int, _: c_int, _: c_int);

	// id, x,y, dim
	fn draw_shield(_: c_int, _: c_double, _: c_double, _: c_double);

	/*
	sprite id, frame index, x, y
	 */
	fn draw_sprite(_: c_uint, _: c_uint, _: c_uint, _: c_uint);
}

lazy_static! {
	// these have to be multiples of 12
    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new(Size{width: 1008, height: 804}));
}

const MOVE_SPEED: f64 = 200.0;

/// Generates a new explosion of the given intensity at the given position.
/// This works best with values between 5 and 25
pub fn make_explosion(particles: &mut Vec<Particle>, position: &Point, intensity: u8, colour_index: ColourIndex) {
    for rotation in itertools_num::linspace(0.0, 2.0 * ::std::f64::consts::PI, 30) {
        for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
            particles.push(Particle::new(Vector::new(position.clone(), rotation), ttl, colour_index));
        }
    }
}

type DeferredShieldDamage = (usize,usize,usize);

unsafe fn handle_collisions(state: &mut State) -> Vec<DeferredShieldDamage> {
	let world = &mut state.world;
	let player = &mut world.player;
	let swarm = &mut world.swarm;
	let bullets = &mut world.bullets;
	let particles = &mut world.particles;
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
			make_explosion(particles, &Point::new(bullet.x(), bullet.y()), 8, ColourIndex::RED);
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
				make_explosion(particles, &Point::new(loc.x,loc.y), 5, ColourIndex::WHITE);
				state.score += points as i32;
				prep_leaderboard_entries(&mut state.leaderboard, "Justin", state.score);
				push_leaderboard_entries(&state.leaderboard);
				world.player_bullet.bullet_type = BulletType::Player(false);
			}

			let ufohit = ufo.check_hit(&world.player_bullet);
			if let Some(hit) = ufohit {
				let points = hit.0;
				let loc = hit.1;
				make_explosion(particles, &Point::new(loc.x,loc.y), 5, ColourIndex::BLUE);
				state.score += points as i32;
				prep_leaderboard_entries(&mut state.leaderboard, "Justin", state.score);
				push_leaderboard_entries(&state.leaderboard);
				world.player_bullet.bullet_type = BulletType::Player(false);
			}
		}
	}


	deferred_shield_damage
}

#[no_mangle]
pub unsafe extern "C" fn update(dt: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();

	match &data.state.game_state {
		GameState::Intro => {
			for (index,shield) in data.state.world.shields.iter_mut().enumerate() {
				shield.reset();
				init_shield(index as i32);
				for i in 0..25 {
					let state = &shield.b[i];
					add_shield_state(index as i32, i as i32, *state as i32);
				}
			}
			if data.input.any {
				data.state.game_state = GameState::Playing;
			}
		},
		GameState::Playing => {
			let radius = data.state.world.swarm.radius;

			if data.input.left && data.state.world.player.x() > radius as f64 {
				*data.state.world.player.x_mut() -= MOVE_SPEED * dt;
			}
			if data.input.right && data.state.world.player.x() < (data.state.world.world_size.width-radius) as f64 {
				*data.state.world.player.x_mut() += MOVE_SPEED * dt;
			};

			// Add bullets
			if data.input.fire {
				if let BulletType::Player(alive) = data.state.world.player_bullet.bullet_type {
					if !alive {
						data.state.world.player_bullet.inplace_new(data.state.world.player.vector.clone(), BulletType::Player(true), 600.);
					}
				}
			}

			// update enemies
			if let Some(bullet) = data.state.world.swarm.update(dt) {
				data.state.world.bullets.push(bullet);
			}

			// update bullets
			for bullet in &mut data.state.world.bullets {
				bullet.update(dt);
			}

			if let BulletType::Player(alive) = data.state.world.player_bullet.bullet_type {
				if alive {
					data.state.world.player_bullet.update(dt);
				}
			}

			// Remove bullets outside the viewport
			{
				let width = data.state.world.world_size.width;
				let height = data.state.world.world_size.height;
				let bullets = &mut data.state.world.bullets;
				bullets.retain(|bullet| {
					let within = bullet.x() > 0. && bullet.x() < width as f64 &&
							bullet.y() > 0. && bullet.y() < height as f64;
					within
				});

				if data.state.world.player_bullet.x() < 0. || data.state.world.player_bullet.x() > width as f64 ||
				data.state.world.player_bullet.y() < 0. || data.state.world.player_bullet.y() > height as f64 {
					data.state.world.player_bullet.bullet_type = BulletType::Player(false);
				}
			}

			let deferred_shield_damage = handle_collisions(&mut data.state);
			{
				let mut_shields = &mut data.state.world.shields;
				for d in deferred_shield_damage {
					let bs = mut_shields[d.0].damage(d.1,d.2);
					update_shield(d.0 as i32, (d.1 as f64 + d.2 as f64 * 5.) as i32, bs as i32);
				}
			}
			data.state.update(dt);
		},
		GameState::Death(_) => {
			if let GameState::Death(ref mut timer) = data.state.game_state {
				*timer -= dt;
				if *timer < 0. {
					data.state.post_death_reset();
					data.state.game_state = GameState::Playing;
				}
			}
		},
		GameState::Win(_) => {
			if let GameState::Win(ref mut timer) = data.state.game_state {
				if *timer >= 0. {
					*timer -= dt;
				} else {
					data.state.reset(ResetType::Next);
					data.state.game_state = GameState::Playing;
				}
			}
		},
		GameState::GameOver(_) => {
			if let GameState::GameOver(ref mut timer) = data.state.game_state {
				if *timer >= 0. {
					*timer -= dt;
				} else {
					if data.input.any {
						data.state.reset(ResetType::New);
						data.state.game_state = GameState::Intro;
					}
				}
			}
		}
	}

	// Update particles
	for particle in &mut data.state.world.particles {
		particle.update(dt);
	}
	{
		let particles = &mut data.state.world.particles;
		particles.retain(|particle| {
			particle.ttl > 0.0
		});
	}
}

unsafe fn draw_swarm(swarm: &Swarm, data: &GameData) {
	// enable to draw bounds
	// let br = swarm.get_bottom_right();
	// draw_bounds(data.screen_top_left_offset.x + swarm.top_left.x * data.game_to_screen, data.screen_top_left_offset.y + swarm.top_left.y * data.game_to_screen, 
	// 			br.x * data.game_to_screen, br.y * data.game_to_screen);
	// is there a better iterator way to do this?
	for i in 0..swarm.num_x {
		for j in 0..swarm.num_y {
			if swarm.alive[j*swarm.num_x+i] {
				let p = swarm.get_enemy_location_screen(i,j, data);
				let index = match j {
					0 => {
						1
					},
					1|2 => {
						2
					},
					// 3|4
					_ => {
						0
					}
				};
				draw_sprite(index, swarm.frame, p.x as u32,p.y as u32);
			}
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn resize(width: c_double, height: c_double) -> c_double {
	let data = &mut DATA.lock().unwrap();
	data.width = width.trunc() as usize;
	data.height = height.trunc() as usize;

	if (data.state.world.world_size.width as f64) < width && (data.state.world.world_size.height as f64) < height {
		data.screen_top_left_offset.x = (width - data.state.world.world_size.width as f64) / 2.;
		data.screen_top_left_offset.y = (height - data.state.world.world_size.height as f64) / 2.;
		data.game_to_screen = 1.;
		return data.game_to_screen;
	}

	// this stuff doesn't work very well...
	if data.state.world.world_size.width as f64 > width {
		data.game_to_screen = width / data.state.world.world_size.width as f64;
		// this isn't quite right; it needs some sort of scaling
		data.screen_top_left_offset.y = (height - data.state.world.world_size.height as f64) / 2.;
	}
	else if data.state.world.world_size.height as f64 > height {
		data.game_to_screen = height / data.state.world.world_size.height as f64;
		// this isn't quite right; it needs some sort of scaling
		data.screen_top_left_offset.x = (width - data.state.world.world_size.width as f64) / 2.;
	}
	data.game_to_screen
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let data = &mut DATA.lock().unwrap();
	let score = data.state.score;
	get_leaderboard_entries(&mut data.state.leaderboard);
	prep_leaderboard_entries(&mut data.state.leaderboard, "Justin", score);
	push_leaderboard_entries(&data.state.leaderboard);
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
		// use geometry::{Advance, Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;


    clear_screen();

	for particle in &world.particles {
		let world_pos = data.world_to_screen(&particle.vector.position);
        draw_particle(world_pos.x, world_pos.y, 5.0 * particle.ttl, particle.get_colour_index());
    }

	// draw_bounds(data.screen_top_left_offset.x, data.screen_top_left_offset.y,
	// 			data.state.world.world_size.width as f64 * data.game_to_screen, data.state.world.world_size.height as f64 * data.game_to_screen);

	match &data.state.game_state {
		GameState::Intro => {
			draw_intro();
		},
		GameState::Playing | GameState::Death(_) | GameState::Win(_) => {
			for bullet in &world.bullets {
				let bp = data.world_to_screen(&bullet.location.position);
				draw_bullet(bp.x, bp.y);
			}
			if let BulletType::Player(alive) = world.player_bullet.bullet_type {
				if alive {
					let bp = data.world_to_screen(&world.player_bullet.location.position);
					draw_player_bullet(bp.x, bp.y);
				}
			}

			let p = data.world_to_screen(&Point{x: world.player.x(), y: world.player.y()});

			if world.player.alive {
				draw_player(p.x, p.y, world.player.dir());
			}

			draw_swarm(&world.swarm, data);

			for (index,shield) in world.shields.iter().enumerate() {
				let screen_pos = data.world_to_screen(&shield.top_left);
				draw_shield(index as i32,screen_pos.x, screen_pos.y, Shield::BLOCK_DIM * data.game_to_screen);
			}

			if world.ufo.active {
				let screen_pos = data.world_to_screen(&world.ufo.position);
				draw_ufo(screen_pos.x, screen_pos.y);
			}
		},
		GameState::GameOver(_) => {
			draw_game_over(data.state.score);
		},
	}

	draw_hud(data.state.score, data.state.lives, data.state.wave);
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}

#[no_mangle]
pub extern "C" fn key_pressed(_: c_char, b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.input.any = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_left(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.input.left = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_right(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.input.right = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_fire(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.input.fire = int_to_bool(b);
}