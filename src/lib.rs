extern crate itertools_num;
use std::os::raw::{c_double, c_int, c_char, c_uint};
use std::sync::Mutex;
use std::f64;

mod input;
mod state;

#[path = "./entities/bullet.rs"]
mod bullet;

#[path = "./entities/player.rs"]
mod player;

#[path = "./entities/swarm.rs"]
mod swarm;

#[path = "./entities/particle.rs"]
mod particle;

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
use crate::bullet::{Bullet,BulletType};
use crate::state::{State, GameData,GameState};
use crate::point::Point;
use crate::vector::Vector;
use crate::particle::Particle;

#[macro_use]
extern crate lazy_static;

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double);
    fn draw_bullet(_: c_double, _: c_double);
    fn draw_particle(_: c_double, _: c_double, _: c_double);
    fn draw_hud(_: c_int, _: c_int);
	fn draw_intro();
	fn draw_game_over(_: c_int);
	fn draw_debug(_: c_double, _: c_double, _: c_double, _: c_double);
	fn draw_bounds(_: c_double, _: c_double, _: c_double, _: c_double);

	// id, 5x5
	fn init_shield(_: c_int);
	fn add_shield_state(_: c_int, _: c_int, _: c_int);

	// id, index, state
	fn update_shield(_: c_int, _: c_int, _: c_int);

	// id, x,y
	fn draw_shield(_: c_int, _: c_double, _: c_double);

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
const BULLETS_PER_SECOND: f64 = 2.0;
const BULLET_RATE: f64 = 1.0 / BULLETS_PER_SECOND;

/// Generates a new explosion of the given intensity at the given position.
/// This works best with values between 5 and 25
pub fn make_explosion(particles: &mut Vec<Particle>, position: &Point, intensity: u8) {
    for rotation in itertools_num::linspace(0.0, 2.0 * ::std::f64::consts::PI, 30) {
        for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
            particles.push(Particle::new(Vector::new(position.clone(), rotation), ttl));
        }
    }
}

fn handle_collisions(state: &mut State) -> bool {
	let world = &mut state.world;
	let player = &mut world.player;
	let swarm = &mut world.swarm;
	let bullets = &mut world.bullets;
	let particles = &mut world.particles;
	let num_bullets = bullets.len();

	let mut score_delta = 0;
	bullets.retain(|bullet| {
		let playerhit = player.check_hit(bullet);
		let swarmhit = swarm.check_hit(bullet);
		if let Some(points) = swarmhit {
			make_explosion(particles, &Point::new(bullet.x(), bullet.y()), 5);
			score_delta += points as i32;
		}
		if playerhit {
			make_explosion(particles, &Point::new(bullet.x(), bullet.y()), 5);
		}
		!playerhit && swarmhit.is_none()
	});
	state.score += score_delta;

	num_bullets != bullets.len()
}

#[no_mangle]
pub extern "C" fn update(dt: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();

	match &data.state.game_state {
		GameState::Intro => {
			if data.input.any {
				data.state.game_state = GameState::Playing;
			}
		},
		GameState::Playing => {
			data.current_time += dt;
			let radius = data.state.world.swarm.radius;

			if data.input.left && data.state.world.player.x() > radius as f64 {
				*data.state.world.player.x_mut() -= MOVE_SPEED * dt;
			}
			if data.input.right && data.state.world.player.x() < (data.state.world.world_size.width-radius) as f64 {
				*data.state.world.player.x_mut() += MOVE_SPEED * dt;
			};

			// Add bullets
			if data.input.fire && data.current_time - data.input.last_shoot > BULLET_RATE {
				data.input.last_shoot = data.current_time;
				data.state.world.bullets.push(Bullet::new(data.state.world.player.vector.clone(), BulletType::Player, 400.));
			}

			// udpate enemies
			if let Some(bullet) = data.state.world.swarm.update(dt) {
				data.state.world.bullets.push(bullet);
			}

			// update bullets
			for bullet in &mut data.state.world.bullets {
				bullet.update(dt);
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

			handle_collisions(&mut data.state);
			data.state.update();
		},
		GameState::Death => {
			// TODO delay
			data.state.post_death_reset();
			data.state.game_state = GameState::Playing;
		},
		GameState::GameOver => {
			if data.input.any {
				data.state.reset();
				data.state.game_state = GameState::Intro;
			}
		}
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
    let world = &data.state.world;

	for (index,shield) in world.shields.iter().enumerate() {
		init_shield(index as i32);
		for i in 0..5 {
			for j in 0..5 {
				let state = &shield.b[j][i];
				add_shield_state(index as i32, (i*5+j) as i32, *state as i32);
			}
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
		// use geometry::{Advance, Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;


    clear_screen();

	for particle in &world.particles {
		let world_pos = data.world_to_screen(&particle.vector.position);
        draw_particle(world_pos.x, world_pos.y, 5.0 * particle.ttl);
    }

	draw_bounds(data.screen_top_left_offset.x, data.screen_top_left_offset.y,
				data.state.world.world_size.width as f64 * data.game_to_screen, data.state.world.world_size.height as f64 * data.game_to_screen);

	match &data.state.game_state {
		GameState::Intro => {
			draw_intro();
		},
		GameState::Playing => {
			for bullet in &world.bullets {
				let bp = data.world_to_screen(&Point{x: bullet.x(), y: bullet.y()});
				draw_bullet(bp.x, bp.y);
			}

			let p = data.world_to_screen(&Point{x: world.player.x(), y: world.player.y()});

			draw_player(p.x, p.y, world.player.dir());

			draw_swarm(&world.swarm, data);

			for (index,shield) in world.shields.iter().enumerate() {
				let screen_pos = data.world_to_screen(&shield.top_left);
				draw_shield(index as i32,screen_pos.x, screen_pos.y);
			}
		},
		GameState::Death => {

		},
		GameState::GameOver => {
			draw_game_over(data.state.score);
		},
	}

	draw_hud(data.state.score, data.state.lives);
	match world.player.alive {
		true => draw_debug(1.,0.,0.,0.),
		false => draw_debug(0.,0.,0.,0.),
	}
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