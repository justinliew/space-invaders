
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
fn draw_hud(_: i32, _: i32, _: i32, _: u8) {}

#[no_mangle]
fn draw_intro() {}

#[no_mangle]
fn draw_sprite(_: u32, _: u32, _: u32, _: u32) {}

#[no_mangle]
fn update_local_score(_: u32, _: u32, _: i32, _: *mut u8) {}

#[no_mangle]
fn new_session();

#[no_mangle]
fn clear_leaderboard() {}
