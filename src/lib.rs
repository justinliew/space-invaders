use std::os::raw::{c_double, c_int};
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
use crate::state::World;

/*
Things to do
- player movement. DONE
- sizing. DONE
- player firing and bullets. DONE
- enemy firing.
- collisions, death.
- game state, losing.
- enemies.
	- proper movement, less smooth.
	- sprites.
- encapsulate for picture-in-picture.
- network story.
- ECS story.
*/


#[macro_use]
extern crate lazy_static;

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double);
    fn draw_enemy(_: c_double, _: c_double, _: c_double);
    fn draw_bullet(_: c_double, _: c_double);
    // fn draw_particle(_: c_double, _: c_double, _: c_double);
    fn draw_score(_: c_double);
	fn draw_debug(_: c_double, _: c_double, _: c_double, _: c_double);
}

lazy_static! {
//    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new(Size{width: 1024, height: 600}, Size{width: 1000, height: 600}));
    static ref DATA: Mutex<GameData> = Mutex::new(GameData::new(Size{width: 1000, height: 600}, Size{width: 1000, height: 600}));
}

struct GameData {
	state: state::State,
	input: Input,
	current_time: f64,
	screen_size: Size,
	in_swarm: bool,
    // state: GameState,
    // actions: Actions,
    // time_controller: TimeController<Pcg32Basic>
}

impl GameData {
	fn new(screen_size: Size, world_size: Size) -> GameData {
		GameData {
			state: state::State::new(world_size),
			input: Input::default(),
			current_time: 0.0,
			screen_size: screen_size,
			in_swarm: false,
			// actions: Actions::default(),
			// time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42]))
		}
	}
}

const MOVE_SPEED: f64 = 300.0;
const BULLETS_PER_SECOND: f64 = 2.0;
const BULLET_RATE: f64 = 1.0 / BULLETS_PER_SECOND;

/*
	let radii = self.radius() + other.radius();
	self.position().squared_distance_to(&other.position()) < radii * radii

					if let Some((index, position)) = enemies.iter().enumerate()
                    .find(|&(_, enemy)| enemy.collides_with(bullet))
                    .map(|(index, enemy)| (index, enemy.position()))
                    {
                        util::make_explosion(particles, &position, 10);
                        enemies.remove(index);
                        false
                    } else {
                    true
                }

*/
fn handle_collisions(world: &mut World) -> bool {
	let swarm = &mut world.swarm;
	let bullets = &world.bullets;

	for bullet in bullets {
		if swarm.check_hit(bullet) {
			return true;
		}
	}
	false
}

#[no_mangle]
pub extern "C" fn update(dt: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();

	data.current_time += dt;

	// Update rocket rotation
	if data.input.left {
		*data.state.world.player.x_mut() -= MOVE_SPEED * dt;
	}
	if data.input.right {
		*data.state.world.player.x_mut() += MOVE_SPEED * dt;
	};

	// Add bullets
	if data.input.fire && data.current_time - data.input.last_shoot > BULLET_RATE {
		data.input.last_shoot = data.current_time;
		data.state.world.bullets.push(Bullet::new(data.state.world.player.vector.clone())); // TODO front
	}

	// udpate enemies
	data.state.world.swarm.update(dt);

	// update bullets
	for bullet in &mut data.state.world.bullets {
		bullet.update(dt);
	}

	data.in_swarm = handle_collisions(&mut data.state.world);

	// CollisionsController::handle_collisions(&mut data.state);
}

unsafe fn draw_swarm(swarm: &Swarm, world_to_screen: (f64,f64)) {
	let radius = swarm.radius as f64 * world_to_screen.0;

	// is there a better iterator way to do this?
	for i in 0..swarm.num_x {
		for j in 0..swarm.num_y {
			if swarm.alive[j*swarm.num_x+i] {
				let p = swarm.get_enemy_location(i,j, world_to_screen);
				draw_enemy(p.x, p.y, radius);
			}
		}
	}
}

#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
	// let data = &mut DATA.lock().unwrap();
	// data.screen_size.width = width as usize;
	// data.screen_size.height = height as usize;
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
		// use geometry::{Advance, Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;

	let world_to_screen = ((data.screen_size.width as f64 / data.state.world.world_size.width as f64),
							(data.screen_size.height as f64 / data.state.world.world_size.height as f64));

    clear_screen();

	for bullet in &world.bullets {
		draw_bullet(bullet.x() * world_to_screen.0, bullet.y() * world_to_screen.1);
		draw_debug(bullet.x(), bullet.y(), world.swarm.top_left.y, world.swarm.bottom_right.y);
	}

    // for enemy in &world.enemies {
    //     draw_enemy(enemy.x(), enemy.y());
    // }

	let px = world.player.x() * world_to_screen.0;
	let py = world.player.y() * world_to_screen.1;

	draw_player(px,py, world.player.dir());

	draw_swarm(&world.swarm, world_to_screen);
	match data.in_swarm {
		true => draw_score(1.0),
		false => draw_score(0.0),
	}

}

fn int_to_bool(i: c_int) -> bool {
    i != 0
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