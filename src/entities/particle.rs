use crate::vector::Vector;
use crate::point::Point;
use crate::game::ColourIndex;

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub vector: Vector,
    pub ttl: f64,
	pub colour_index: ColourIndex,
}

//derive_position_direction!(Particle);

impl Particle {
    /// Create a particle with the given vector and time to live in seconds
    pub fn new(vector: Vector, ttl: f64, colour_index: ColourIndex) -> Particle {
        Particle { vector: vector, ttl: ttl, colour_index: colour_index }
    }

	pub fn x_mut(&mut self) -> &mut f64 { &mut self.vector.position.x }
	pub fn y_mut(&mut self) -> &mut f64 { &mut self.vector.position.y }

	pub fn dir(&self) -> f64 { self.vector.direction }

    /// Update the particle
    pub fn update(&mut self, elapsed_time: f64) {
        self.ttl -= elapsed_time;
        let speed = 1000.0 * self.ttl * self.ttl;
        self.advance(elapsed_time * speed);
    }

	pub fn get_colour_index(&self) -> i32 {
		match self.colour_index {
			ColourIndex::BLUE => 1,
			ColourIndex::RED => 2,
			ColourIndex::WHITE => 0,
		}
	}

    fn advance(&mut self, units: f64) {
        *self.x_mut() += self.dir().cos() * units;
        *self.y_mut() += self.dir().sin() * units;
    }
}

/// Generates a new explosion of the given intensity at the given position.
/// This works best with values between 5 and 25
pub fn make_explosion(particles: &mut Vec<Particle>, position: &Point, intensity: u8, colour_index: ColourIndex) {
    for rotation in itertools_num::linspace(0.0, 2.0 * ::std::f64::consts::PI, 15) {
        for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
            particles.push(Particle::new(Vector::new(position.clone(), rotation), ttl, colour_index));
        }
    }
}
