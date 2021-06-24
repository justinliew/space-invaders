
#[no_mangle]
fn clear_screen() {}

#[no_mangle]
fn draw_player(_: f64, _: f64, _: f64) {}

#[no_mangle]
fn draw_bullet(_: f64, _: f64) {}

#[no_mangle]
fn draw_player_bullet(_: f64, _: f64) {}

#[no_mangle]
fn draw_particle(_: f64, _: f64, _: f64, _: i32) {}

#[no_mangle]
fn draw_ufo(_: f64, _: f64) {}

#[no_mangle]
fn draw_hud(_: i32, _: i32, _: i32) {}

#[no_mangle]
fn draw_intro() {}

#[no_mangle]
fn draw_game_over(_: i32) {}

#[no_mangle]
fn draw_shield(_: i32, _: f64, _: f64, _: f64) {}

#[no_mangle]
fn draw_sprite(_: u32, _: u32, _: u32, _: u32) {}

#[no_mangle]
fn update_leaderboard_entry(_: u32, _: u32, _: i32, _: *mut u8) {}

#[no_mangle]
fn clear_leaderboard() {}

#[no_mangle]
fn init_shield(_: i32) {}

#[no_mangle]
fn add_shield_state(_: i32, _: i32, _: i32) {}

#[no_mangle]
fn update_shield(_: i32, _: i32, _: i32) {}
