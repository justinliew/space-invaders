extern crate itertools_num;
use std::os::raw::{c_double, c_int, c_char};
use std::sync::Mutex;
use std::ffi::CString;

mod input;
mod render;
mod game;

#[path = "./entities/bullet.rs"]
mod bullet;

#[path = "./entities/particle.rs"]
mod particle;

#[path = "./entities/player.rs"]
mod player;

#[path = "./entities/swarm.rs"]
mod swarm;

#[path = "./entities/world.rs"]
mod world;

#[path = "./core/collision.rs"]
mod collision;

#[path = "./core/point.rs"]
mod point;

#[path = "./core/vector.rs"]
mod vector;

#[path = "./core/size.rs"]
mod size;

#[path = "./core/log.rs"]
mod log;

use crate::size::WorldSize;
use crate::game::GameData;
use crate::render::RenderData;

#[macro_use]
extern crate lazy_static;


lazy_static! {
	static ref RENDER: Mutex<RenderData> = Mutex::new(RenderData::new());
    static ref GAME: Mutex<GameData> = Mutex::new(GameData::new(WorldSize::new(1008.,804.), RENDER.lock().unwrap().sender.clone()));
}

#[no_mangle]
pub unsafe extern "C" fn update(dt: c_double) {
    let data: &mut GameData = &mut GAME.lock().unwrap();

	data.game.update(&data.input, dt);
}

#[no_mangle]
pub unsafe extern "C" fn resize(width: c_double, height: c_double) -> c_double {
	let render = &mut RENDER.lock().unwrap();
	let data = &mut GAME.lock().unwrap();
	let world_size = data.game.world.world_size;

	render.resize(world_size, width, height)
}

#[no_mangle]
pub unsafe extern "C" fn init() {
	// TODO - these have to be here. Why?
    let _data = &mut GAME.lock().unwrap();
	let _render = &mut RENDER.lock().unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn draw(dt: c_double) {
    let render = &mut RENDER.lock().unwrap();
	let data = &mut GAME.lock().unwrap();
    let game = &data.game;

	render.draw(game.game_state, game, dt);
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}

#[no_mangle]
pub extern "C" fn key_pressed(_: c_char, b: c_int) {
    let data = &mut GAME.lock().unwrap();
    data.input.any = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_left(b: c_int) {
    let data = &mut GAME.lock().unwrap();
    data.input.left = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_right(b: c_int) {
    let data = &mut GAME.lock().unwrap();
    data.input.right = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_fire(b: c_int) {
    let data = &mut GAME.lock().unwrap();
    data.input.fire = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_alt(b: c_int) {
    let data = &mut GAME.lock().unwrap();
    data.input.alt = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    let _ = unsafe { CString::from_raw(ptr) };
}
