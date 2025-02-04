//use rand::Rng;

//use super::Size;

use std::ops::{Add, Sub, Mul, Div};

/// A `Point` represents a position in space
#[derive(Clone, Default, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    /// Returns a new `Point` with the given coordinates
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

/// Implements '==' for Point, as well as its inverse '!='
impl PartialEq for Point {
    fn eq (&self, _rhs: &Self) -> bool {
        (self.x == _rhs.x) && (self.y == _rhs.y)
    }
}

/// Implements the '+' operator for Point + Point
impl Add for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

/// Implements the '+' operator for Point + f64
impl Add<f64> for Point {
    type Output = Point;

    fn add(self, _rhs: f64) -> Point {
        Point {
            x: self.x + _rhs,
            y: self.y + _rhs,
        }
    }
}

/// Implements the '-' operator for Point - Point
impl Sub for Point {
    type Output = Point;

    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

/// Implements the '-' operator for Point - f64
impl Sub<f64> for Point {
    type Output = Point;

    fn sub(self, _rhs: f64) -> Point {
        Point {
            x: self.x - _rhs,
            y: self.y - _rhs,
        }
    }
}

/// Implements the '*' operator for Point * Point
impl Mul for Point {
    type Output = Point;

    fn mul(self, _rhs: Point) -> Point {
        Point {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

/// Implements the '*' operator for Point * f64
impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, _rhs: f64) -> Point {
        Point {
            x: self.x * _rhs,
            y: self.x * _rhs,
        }
    }
}

/// Implements the '/' operator for Point / Point
impl Div for Point {
    type Output = Point;

    fn div(self, _rhs: Point) -> Point {
        assert!(_rhs.x != 0f64);
        assert!(_rhs.y != 0f64);
        Point {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        }
    }
}

/// Implements the '/' operator for Point / f64:
impl Div<f64> for Point {
    type Output = Point;

    fn div(self, _rhs: f64) -> Point {
        assert!(_rhs != 0f64);
        Point {
            x: self.x / _rhs,
            y: self.y / _rhs,
        }
    }
}