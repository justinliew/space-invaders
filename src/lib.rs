extern crate itertools_num;
use std::os::raw::{c_double, c_int, c_char, c_uint};
use std::sync::Mutex;
use std::f64;

mod input;
mod render;
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

#[path = "./core/collision.rs"]
mod collision;

#[path = "./core/point.rs"]
mod point;

#[path = "./core/vector.rs"]
mod vector;

#[path = "./core/size.rs"]
mod size;

use crate::swarm::Swarm;
use crate::size::{WorldSize};
use crate::bullet::{BulletType};
use crate::state::{GameData,GameState};
use crate::point::Point;
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

	// id, x,y, dim
	fn draw_shield(_: c_int, _: c_double, _: c_double, _: c_double);

	// sprite id, frame index, x, y
	fn draw_sprite(_: c_uint, _: c_uint, _: c_uint, _: c_uint);
}

lazy_static! {
	// these have to be multiples of 12
    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new(WorldSize::new(1008.,804.)));
}

#[no_mangle]
pub unsafe extern "C" fn update(dt: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();

	data.state.update(&data.input, dt);

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
	let render = &data.render;

	// enable to draw bounds
	// let br = swarm.get_bottom_right();
	// draw_bounds(data.screen_top_left_offset.x + swarm.top_left.x * data.game_to_screen, data.screen_top_left_offset.y + swarm.top_left.y * data.game_to_screen, 
	// 			br.x * data.game_to_screen, br.y * data.game_to_screen);
	// is there a better iterator way to do this?
	for i in 0..swarm.num_x {
		for j in 0..swarm.num_y {
			if swarm.alive[j*swarm.num_x+i] {
				let p = render.world_to_screen(&swarm.get_enemy_location(i,j));
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
	let world_size = data.state.world.world_size;
	let render = &mut data.render;

	render.resize(world_size, width, height)
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
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;
	let render = &data.render;

    clear_screen();

	for particle in &world.particles {
		let world_pos = render.world_to_screen(&particle.vector.position);
        draw_particle(world_pos.x, world_pos.y, 5.0 * particle.ttl, particle.get_colour_index());
    }

	match &data.state.game_state {
		GameState::Intro => {
			draw_intro();
		},
		GameState::Playing | GameState::Death(_) | GameState::Win(_) => {
			for bullet in &world.bullets {
				let bp = render.world_to_screen(&bullet.location.position);
				draw_bullet(bp.x, bp.y);
			}
			if let BulletType::Player(alive) = world.player_bullet.bullet_type {
				if alive {
					let bp = render.world_to_screen(&world.player_bullet.location.position);
					draw_player_bullet(bp.x, bp.y);
				}
			}

			let p = render.world_to_screen(&Point{x: world.player.x(), y: world.player.y()});

			if world.player.alive {
				draw_player(p.x, p.y, world.player.dir());
			}

			draw_swarm(&world.swarm, data);

			for (index,shield) in world.shields.iter().enumerate() {
				let screen_pos = render.world_to_screen(&shield.top_left);
				draw_shield(index as i32, screen_pos.x, screen_pos.y, Shield::BLOCK_DIM * data.render.game_to_screen);
			}

			if world.ufo.active {
				let screen_pos = render.world_to_screen(&world.ufo.position);
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