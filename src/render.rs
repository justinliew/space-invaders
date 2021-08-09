use std::os::raw::{c_double, c_int, c_uint};

use crate::point::Point;
use crate::size::WorldSize;
use crate::game::{Game, GameEvent, GameState};
use crate::bullet::BulletType;
use crate::swarm::Swarm;
use crate::shield::Shield;
use crate::particle::{make_explosion, Particle};

use std::sync::mpsc;
use std::sync::mpsc::{Receiver,Sender};

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

	fn update_local_score(_: c_int);

}

pub struct RenderData {
	pub screen_top_left_offset: Point,
	pub game_to_screen: f64,
	pub width: usize,
	pub height: usize,
    pub particles: Vec<Particle>,
	receiver: Receiver<GameEvent>,
	pub sender: Sender<GameEvent>,
}

impl RenderData {
	pub fn new() -> Self {
		let (tx,rx) = mpsc::channel();
		RenderData{
			screen_top_left_offset: Point::new(0.0,0.0),
			game_to_screen: 1.,
			width: 1024,
			height: 768,
            particles: Vec::with_capacity(1000),
			receiver: rx,
			sender: tx,
		}
	}

	pub fn world_to_screen(&self, in_point: &Point) -> Point {
		Point{
			x: (in_point.x + self.screen_top_left_offset.x) * self.game_to_screen,
			y: (in_point.y + self.screen_top_left_offset.y) * self.game_to_screen,
		}
	}

	pub fn resize(&mut self, world_size: WorldSize, width: f64, height: f64) -> f64 {
		self.width = width.trunc() as usize;
		self.height = height.trunc() as usize;

		if world_size.width < width && world_size.height < height {
			self.screen_top_left_offset.x = (width - world_size.width) / 2.;
			self.screen_top_left_offset.y = (height - world_size.height) / 2.;
			self.game_to_screen = 1.;
			return self.game_to_screen;
		}

		// this stuff doesn't work very well...
		if world_size.width > width {
			self.game_to_screen = width / world_size.width;
			// this isn't quite right; it needs some sort of scaling
			self.screen_top_left_offset.y = (height - world_size.height) / 2.;
		}
		else if world_size.height > height {
			self.game_to_screen = height / world_size.height;
			// this isn't quite right; it needs some sort of scaling
			self.screen_top_left_offset.x = (width - world_size.width) / 2.;
		}
		self.game_to_screen
	}

	unsafe fn draw_swarm(&self, swarm: &Swarm) {
		// enable to draw bounds
		// let br = swarm.get_bottom_right();
		// draw_bounds(data.screen_top_left_offset.x + swarm.top_left.x * data.game_to_screen, data.screen_top_left_offset.y + swarm.top_left.y * data.game_to_screen, 
		// 			br.x * data.game_to_screen, br.y * data.game_to_screen);

		// is there a better iterator way to do this?
		for i in 0..swarm.num_x {
			for j in 0..swarm.num_y {
				if swarm.alive[j*swarm.num_x+i] {
					let p = self.world_to_screen(&swarm.get_enemy_location(i,j));
					let index = match j {
						0 => 1,
						1|2 => 2,
						_ => 0, // 3|4
					};
					draw_sprite(index, swarm.frame, p.x as u32,p.y as u32);
				}
			}
		}
	}

	unsafe fn handle_game_event(&mut self, event: GameEvent) {
		match event {
			GameEvent::ScoreChanged(i) => {
				update_local_score(i);
			},
			GameEvent::EntityDied(p,c) => {
				let particles = &mut self.particles;
				make_explosion(particles, &p, 6, c);
			}
		}
	}

	pub unsafe fn draw(&mut self, game_state: GameState, game: &Game, dt: f64) {
		let world = &game.world;
		clear_screen();

		match self.receiver.try_recv() {
			Ok(event) => {
				self.handle_game_event(event);
			},
			Err(_) => {

			}
		}

		{
			let particles = &mut self.particles;
			particles.retain(|particle| {
				particle.ttl > 0.0
			});
		}

		for particle in &mut self.particles {
			particle.update(dt);
		}

		for particle in &self.particles {
			let world_pos = self.world_to_screen(&particle.vector.position);
			draw_particle(world_pos.x, world_pos.y, 5.0 * particle.ttl, particle.get_colour_index());
		}

		match game_state {
			GameState::Intro(_) => {
				draw_intro();
			},
			GameState::Playing | GameState::Death(_) | GameState::Win(_) => {
				for bullet in &world.bullets {
					let bp = self.world_to_screen(&bullet.location.position);
					draw_bullet(bp.x, bp.y);
				}
				if let BulletType::Player(alive) = world.player_bullet.bullet_type {
					if alive {
						let bp = self.world_to_screen(&world.player_bullet.location.position);
						draw_player_bullet(bp.x, bp.y);
					}
				}

				let p = self.world_to_screen(&Point{x: world.player.x(), y: world.player.y()});

				if world.player.alive {
					draw_player(p.x, p.y, world.player.dir());
				}

				self.draw_swarm(&world.swarm);

				for (index,shield) in world.shields.iter().enumerate() {
					let screen_pos = self.world_to_screen(&shield.top_left);
					draw_shield(index as i32, screen_pos.x, screen_pos.y, Shield::BLOCK_DIM * self.game_to_screen);
				}

				if world.ufo.active {
					let screen_pos = self.world_to_screen(&world.ufo.position);
					draw_ufo(screen_pos.x, screen_pos.y);
				}
			},
			GameState::GameOver(_) => {
				draw_game_over(game.score);
			},
		}

		draw_hud(game.score, game.lives, game.wave);
	}
}

