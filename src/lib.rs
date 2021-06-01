use std::os::raw::{c_double};

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    // fn draw_player(_: c_double, _: c_double, _: c_double);
    // fn draw_enemy(_: c_double, _: c_double);
    // fn draw_bullet(_: c_double, _: c_double);
    // fn draw_particle(_: c_double, _: c_double, _: c_double);
    fn draw_score(_: c_double);
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
		// use geometry::{Advance, Position};
    // let data = &mut DATA.lock().unwrap();
    // let world = &data.state.world;

    clear_screen();
    // for particle in &world.particles {
    //     draw_particle(particle.x(), particle.y(), 5.0 * particle.ttl);
    // }

    // for bullet in &world.bullets {
    //     draw_bullet(bullet.x(), bullet.y());
    // }

    // for enemy in &world.enemies {
    //     draw_enemy(enemy.x(), enemy.y());
    // }
    // draw_player(world.player.x(), world.player.y(), world.player.direction());
    draw_score(66.0);
}