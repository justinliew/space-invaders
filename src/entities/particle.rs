use crate::vector::Vector;

//use geometry::Advance;

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub vector: Vector,
    pub ttl: f64
}

//derive_position_direction!(Particle);

impl Particle {
    /// Create a particle with the given vector and time to live in seconds
    pub fn new(vector: Vector, ttl: f64) -> Particle {
        Particle { vector: vector, ttl: ttl }
    }

	// TODO derive_position_direction
	pub fn x(&self) -> f64 { self.vector.position.x }
	pub fn x_mut(&mut self) -> &mut f64 { &mut self.vector.position.x }
	pub fn y(&self) -> f64 { self.vector.position.y }
	pub fn y_mut(&mut self) -> &mut f64 { &mut self.vector.position.y }

	pub fn dir(&self) -> f64 { self.vector.direction }

    /// Update the particle
    pub fn update(&mut self, elapsed_time: f64) {
        self.ttl -= elapsed_time;
        let speed = 1000.0 * self.ttl * self.ttl;
        self.advance(elapsed_time * speed);
    }

    fn advance(&mut self, units: f64) {
        *self.x_mut() += self.dir().cos() * units;
        *self.y_mut() += self.dir().sin() * units;
    }
}