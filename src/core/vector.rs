use std::f64;

use crate::point::Point;

/// A `Vector`
#[derive(Clone, Default)]
pub struct Vector {
    /// The position of the vector
    pub position: Point,
    /// The direction angle, in radians
    pub direction: f64
}

impl Vector {
    /// Returns a new `Vector`
    pub fn new(position: Point, direction: f64) -> Vector {
        Vector { position: position, direction: direction }
    }
}
