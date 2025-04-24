use std::os::raw::{c_double, c_int, c_uchar, c_uint};

use crate::bullet::Ability;
use crate::game::{Condition, Game, GameEvent, GameState};
use crate::log::*;
use crate::particle::{make_explosion, Particle};
use crate::point::Point;
use crate::shield::Shield;
use crate::size::WorldSize;
use crate::swarm::Swarm;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double, _: c_int);
    fn draw_bullet(_: c_double, _: c_double);
    fn draw_player_bullet(_: c_double, _: c_double, _: c_double, _: c_int);
    fn draw_particle(_: c_double, _: c_double, _: c_double, _: c_int);
    fn draw_ufo(_: c_double, _: c_double);
    fn draw_hud(_: c_int, _: c_int, _: c_int);
    fn draw_intro();
    fn draw_condition_warning(_: c_uchar, _: c_int, _: c_int);
    fn draw_name_picker(_: c_int, _: c_int);

    fn draw_fastly_treatment(_: c_double);
    fn reset_fastly_treatment();

    fn draw_bounds(_: c_double, _: c_double, _: c_double, _: c_double);

    fn draw_line(_: c_double, _: c_double, _: c_double, _: c_double, _: c_int);

    // id, x,y, dim
    fn draw_shield(_: c_int, _: c_double, _: c_double, _: c_double, _: c_int);

    // sprite id, frame index, x, y
    fn draw_sprite(_: c_uint, _: c_uint, _: c_uint, _: c_uint);

    fn update_local_score(_: c_int);

}

pub struct ConditionText {
    pub on: bool,
    pub blink_countdown: f64,
    pub countdown: f64,
}

impl ConditionText {
    pub fn update(&mut self, dt: f64) {
        self.countdown -= dt;
        match self.on {
            false => {
                if self.blink_countdown <= 0. {
                    self.blink_countdown = 0.2;
                    self.on = true;
                }
                self.blink_countdown -= dt;
            }
            true => {
                if self.blink_countdown <= 0. {
                    self.blink_countdown = 0.2;
                    self.on = false;
                }
                self.blink_countdown -= dt;
            }
        }
    }
}

pub struct RenderData {
    pub screen_top_left_offset: Point,
    pub game_to_screen: f64,
    pub width: usize,
    pub height: usize,
    pub condition_text: Option<ConditionText>,
    pub particles: Vec<Particle>,
    receiver: Receiver<GameEvent>,
    pub sender: Sender<GameEvent>,
}

impl RenderData {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        RenderData {
            screen_top_left_offset: Point::new(0.0, 0.0),
            game_to_screen: 1.,
            width: 1024,
            height: 768,
            condition_text: None,
            particles: Vec::with_capacity(1000),
            receiver: rx,
            sender: tx,
        }
    }

    fn enable_condition_text(&mut self) {
        self.condition_text = Some(ConditionText {
            on: false,
            countdown: 3.,
            blink_countdown: 0.,
        });
    }

    fn disable_condition_text(&mut self) {
        self.condition_text = None;
    }

    pub fn world_to_screen(&self, in_point: &Point) -> Point {
        Point {
            x: (in_point.x + self.screen_top_left_offset.x) * self.game_to_screen,
            y: (in_point.y + self.screen_top_left_offset.y) * self.game_to_screen,
        }
    }

    pub fn resize(&mut self, world_size: WorldSize, width: f64, height: f64) -> f64 {
        self.width = width.trunc() as usize;
        self.height = height.trunc() as usize;

//        if world_size.width < width && world_size.height < height {
		self.screen_top_left_offset.x = (width - world_size.width) / 2.;
		self.screen_top_left_offset.y = (height - world_size.height) / 2.;
		self.game_to_screen = 1.;

		return self.game_to_screen;
//        }

        // this stuff doesn't work very well...
        // if world_size.width > width {
        //     self.game_to_screen = width / world_size.width;
        //     // this isn't quite right; it needs some sort of scaling
        //     self.screen_top_left_offset.y = (height - world_size.height) / 2.;
		// 	log(&format!("setting y"));
        // } else if world_size.height > height {
        //     self.game_to_screen = height / world_size.height;
        //     // this isn't quite right; it needs some sort of scaling
        //     self.screen_top_left_offset.x = (width - world_size.width) / 2.;
		// 	log(&format!("setting x"));
        // }
		// log(&format!("game to screen {:?}", self.screen_top_left_offset));
        self.game_to_screen
    }

    unsafe fn draw_swarm(&self, swarm: &Swarm) {
        // enable to draw bounds
        // let br = swarm.get_bottom_right();
        // draw_bounds(self.screen_top_left_offset.x + swarm.top_left.x * self.game_to_screen, self.screen_top_left_offset.y + swarm.top_left.y * self.game_to_screen,
        // 	self.screen_top_left_offset.x + br.x * self.game_to_screen, self.screen_top_left_offset.y + br.y * self.game_to_screen);

        // draw_line(self.screen_top_left_offset.x + swarm.lhs*self.game_to_screen,self.screen_top_left_offset.y,self.screen_top_left_offset.x + swarm.lhs*self.game_to_screen, 500., 1);
        // draw_line(self.screen_top_left_offset.x + swarm.rhs*self.game_to_screen,self.screen_top_left_offset.y,self.screen_top_left_offset.x + swarm.rhs*self.game_to_screen, 500., 1);

        // is there a better iterator way to do this?
        for i in 0..swarm.num_x {
            for j in 0..swarm.num_y {
                if swarm.alive[j * swarm.num_x + i] {
                    let p = self.world_to_screen(&swarm.get_enemy_location(i, j));
                    let index = match j {
                        0 => 1,
                        1 | 2 => 2,
                        _ => 0, // 3|4
                    };
                    draw_sprite(index, swarm.frame, p.x as u32, p.y as u32);
                }
            }
        }
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
            GameEvent::Condition(_c) => {
                self.enable_condition_text();
            }
        }
    }

    pub unsafe fn draw(&mut self, game_state: GameState, game: &Game, dt: f64) {
        let world = &game.world;
        clear_screen();

        let warning_loc = self.world_to_screen(&Point {
            x: self.width as f64 / 4.,
            y: 400.,
        });

        if let Some(ct) = &mut self.condition_text {
            ct.update(dt);
            if ct.on {
                // TODO reusing condition as wave
                draw_condition_warning(game.wave as u8, warning_loc.x as i32, warning_loc.y as i32);
            }
            if ct.countdown <= 0. {
                self.disable_condition_text();
            }
        }

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
            | GameState::Win(_)
            | GameState::GameOverFastlyTreatment(_) => {
                if !matches!(game_state, GameState::GameOverFastlyTreatment(_)) {
                    for bullet in world.get_bullets() {
                        let bp = self.world_to_screen(&bullet.location.position);
                        draw_bullet(bp.x, bp.y);
                    }
                    let player_bullet = world.get_player_bullet();
                    if player_bullet.active {
                        // TODO break up `draw_player_bullet` into multiple functions?
                        let bp = self.world_to_screen(&player_bullet.location.position);
                        draw_player_bullet(
                            bp.x,
                            bp.y,
                            player_bullet.facing,
                            (player_bullet.ability == Ability::Bomb) as i32,
                        );
                    }

                    if world.get_ufo().active {
                        let screen_pos = self.world_to_screen(&world.get_ufo().position);
                        draw_ufo(screen_pos.x, screen_pos.y);
                    }

                    for (index, shield) in world.get_active_shields().iter().enumerate() {
                        let screen_pos = self.world_to_screen(&shield.top_left);
                        let condition = game
                            .conditions
                            .iter()
                            .find(|v| *v == &Condition::Shields)
                            .is_some();
                        draw_shield(
                            index as i32,
                            screen_pos.x,
                            screen_pos.y,
                            Shield::BLOCK_DIM * self.game_to_screen,
                            condition as i32,
                        );
                    }
                }

                let player = world.get_player();
                let p = self.world_to_screen(&Point {
                    x: player.x(),
                    y: player.y(),
                });

                if player.alive {
                    draw_player(p.x, p.y, player.dir(), 0);
                }

                self.draw_swarm(&world.get_swarm());

                // unsafe {
				// 	let tl = self.world_to_screen(&Point::new(0.,0.));
                //     draw_bounds(tl.x,tl.y, self.width as f64, self.height as f64);
                // }

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
