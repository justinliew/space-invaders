use std::os::raw::{c_double, c_int, c_uint};

use crate::game::{Game, GameEvent, GameState};
use crate::particle::{make_explosion, Particle};
use crate::point::Point;
use crate::size::WorldSize;
use crate::swarm::Swarm;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double, _: c_int);
    fn draw_bullet(_: c_double, _: c_double);
    fn draw_player_bullet(_: c_double, _: c_double, _: c_double);
    fn draw_particle(_: c_double, _: c_double, _: c_double, _: c_int);
    fn draw_hud(_: c_int, _: c_int, _: c_int);
    fn draw_bg(_: c_int, _: c_double, _: c_double, _: c_int, _: c_int);
    fn draw_line(_: c_double, _: c_double, _: c_double, _: c_double, _: c_int);
    fn draw_intro();
    fn draw_name_picker(_: c_int, _: c_int);

    fn draw_fastly_treatment(_: c_double);
    fn reset_fastly_treatment();

    // sprite id, frame index, x, y
    fn draw_sprite(_: c_uint, _: c_uint, _: c_uint, _: c_uint);

    fn update_local_score(_: c_int);

}

pub struct RenderData {
    pub width: usize,
    pub height: usize,
    pub particles: Vec<Particle>,
    receiver: Receiver<GameEvent>,
    pub sender: Sender<GameEvent>,
}

impl RenderData {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        RenderData {
            width: 1024,
            height: 768,
            particles: Vec::with_capacity(1000),
            receiver: rx,
            sender: tx,
        }
    }

	pub fn world_to_screen(&self, in_point: &Point) -> Point {
		Point{
			x: (in_point.x + 100.),
			y: (in_point.y + 100.),
		}
	}

    pub fn resize(&mut self, world_size: WorldSize, width: f64, height: f64) {
        self.width = width.trunc() as usize;
        self.height = height.trunc() as usize;
    }

    unsafe fn handle_game_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::ScoreChanged(i) => {
                update_local_score(i);
            }
            GameEvent::EntityDied(p, c) => {
                let particles = &mut self.particles;
                make_explosion(particles, &p, 6, c);
            }
        }
    }

    pub unsafe fn draw(&mut self, game_state: GameState, game: &Game, dt: f64) {
        let world = &game.world;
        clear_screen();

        let top_left = self.world_to_screen(&Point { x: 0., y: 0. });
        draw_bg(world.scrolly as i32, top_left.x, top_left.y, game.world.world_size.width as i32, game.world.world_size.height as i32);
        draw_line(top_left.x, top_left.y, top_left.x, top_left.y + 1000., 1);
        draw_line(
            top_left.x,
            top_left.y + 1000.,
            top_left.x + 1000.,
            top_left.y + 1000.,
            1,
        );
        draw_line(
            top_left.x + 1000.,
            top_left.y + 1000.,
            top_left.x + 1000.,
            top_left.y,
            1,
        );
        draw_line(top_left.x + 1000., top_left.y, top_left.x, top_left.y, 1);

        match self.receiver.try_recv() {
            Ok(event) => {
                self.handle_game_event(event);
            }
            Err(_) => {}
        }

        {
            let particles = &mut self.particles;
            particles.retain(|particle| particle.ttl > 0.0);
        }

        for particle in &mut self.particles {
            particle.update(dt);
        }

        for particle in &self.particles {
            let world_pos = self.world_to_screen(&particle.vector.position);
            draw_particle(
                world_pos.x,
                world_pos.y,
                5.0 * particle.ttl,
                particle.get_colour_index(),
            );
        }

        match game_state {
            GameState::Intro(_) => {
                draw_intro();
            }
            GameState::Playing
            | GameState::Death(_)
            | GameState::_Win(_)
            | GameState::GameOverFastlyTreatment(_) => {
                if !matches!(game_state, GameState::GameOverFastlyTreatment(_)) {
                    for bullet in world.get_bullets() {
                        let bp = self.world_to_screen(&bullet.location.position);
                        draw_bullet(bp.x, bp.y);
                    }
                    for player_bullet in world.get_player_bullets() {
                        let bp = self.world_to_screen(&player_bullet.location.position);
                        draw_player_bullet(bp.x, bp.y, player_bullet.facing);                        
                    }
                }

                let player = world.get_player();
                let p = self.world_to_screen(&player.vector.position);

                if player.alive {
                    draw_player(p.x, p.y, player.dir(), 0);
                }

                if let GameState::GameOverFastlyTreatment(t) = game_state {
                    draw_fastly_treatment(t);
                }
            }
            GameState::CheckHighScore => {
                reset_fastly_treatment();
            }
            GameState::WaitHighScore => {}
            GameState::ShowHighScore(_, _, _) => {
                draw_name_picker(game.letter_index, game.cur_letter);
            }
            GameState::GameOver(_) => {}
        }

        draw_hud(game.score, game.lives, game.wave);
    }
}
