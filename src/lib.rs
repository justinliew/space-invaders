use std::os::raw::{c_double, c_int, c_char};
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

#[path = "./core/point.rs"]
mod point;
#[path = "./core/vector.rs"]
mod vector;

#[path = "./core/size.rs"]
mod size;

use crate::input::Input;
use crate::swarm::Swarm;
use crate::size::Size;
use crate::bullet::Bullet;
use crate::state::{GameData,World,GameState};
use crate::point::Point;

#[macro_use]
extern crate lazy_static;

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double);
    fn draw_enemy(_: c_double, _: c_double, _: c_double);
    fn draw_bullet(_: c_double, _: c_double);
    // fn draw_particle(_: c_double, _: c_double, _: c_double);
    fn draw_hud(_: c_int, _: c_int);
	fn draw_intro();
	fn draw_game_over(_: c_int);
	fn draw_debug(_: c_double, _: c_double, _: c_double, _: c_double);
	fn draw_bounds(_: c_double, _: c_double, _: c_double, _: c_double);
}

lazy_static! {
//    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new(Size{width: 1024, height: 600}, Size{width: 1000, height: 600}));
    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new(Size{width: 1000, height: 800}));
}

const MOVE_SPEED: f64 = 300.0;
const BULLETS_PER_SECOND: f64 = 2.0;
const BULLET_RATE: f64 = 1.0 / BULLETS_PER_SECOND;

fn handle_collisions(world: &mut World) -> bool {
	let swarm = &mut world.swarm;
	let bullets = &mut world.bullets;
	let num_bullets = bullets.len();

	bullets.retain(|bullet| {
		let hit = swarm.check_hit(bullet);
		!hit
	});

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
				data.state.world.bullets.push(Bullet::new(data.state.world.player.vector.clone())); // TODO front
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
					let within = bullet.x() > 0. && bullet.x() < width as f64&&
							bullet.y() > 0. && bullet.y() < height as f64;
					within
				});
			}

			handle_collisions(&mut data.state.world);
			data.state.update();
		},
		GameState::Death => {

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
	let radius = swarm.radius as f64 * data.game_to_screen;

	// is there a better iterator way to do this?
	for i in 0..swarm.num_x {
		for j in 0..swarm.num_y {
			if swarm.alive[j*swarm.num_x+i] {
				let p = swarm.get_enemy_location_screen(i,j, data);
				draw_enemy(p.x, p.y, radius);
			}
		}
	}
	let lowest = swarm.get_lowest_alive().unwrap();
}

#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
	let data = &mut DATA.lock().unwrap();
	data.width = width.trunc() as usize;
	data.height = height.trunc() as usize;

	if (data.state.world.world_size.width as f64) < width && (data.state.world.world_size.height as f64) < height {
		data.screen_top_left_offset.x = (width - data.state.world.world_size.width as f64) / 2.;
		data.screen_top_left_offset.y = (height - data.state.world.world_size.height as f64) / 2.;
		return;
	}

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

}

#[no_mangle]
pub unsafe extern "C" fn draw() {
		// use geometry::{Advance, Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;


    clear_screen();

	draw_bounds(data.screen_top_left_offset.x, data.screen_top_left_offset.y, data.state.world.world_size.width as f64 * data.game_to_screen, data.state.world.world_size.height as f64 * data.game_to_screen);

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
		},
		GameState::Death => {

		},
		GameState::GameOver => {
			draw_game_over(data.state.score);
		},
	}

	draw_hud(data.state.score, data.state.lives);
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}

#[no_mangle]
pub extern "C" fn key_pressed(k: c_char, b: c_int) {
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