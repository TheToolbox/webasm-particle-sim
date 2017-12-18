use std::ops::{AddAssign, Div, Mul, Sub, Add};
use std::f64;

/// A `Vector`
#[derive(Clone, Copy, Default)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    /// Returns a new `Vector`
    pub fn _new(x: f64, y: f64) -> Vector2D {
        Vector2D { x: x, y: y }
    }

    pub fn _from_mag_angle(magnitude: f64, angle: f64) -> Vector2D {
        let x = magnitude * angle.cos();
        let y = magnitude * angle.sin();
        Vector2D { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    ///Consumes and returns normalized form
    pub fn _normalize(self) -> Vector2D {
        self / self.magnitude()
    }

    /*pub fn random(mut rng: &mut Rng, range: &Range<f64>) -> Vector2D {
        Vector2D {
            x: range.ind_sample(&mut rng),
            y: range.ind_sample(&mut rng),
        }
    }*/
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, _rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, _rhs: f64) -> Vector2D {
        Vector2D {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

/// Implements the '/' operator for Point / f64:
impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, _rhs: f64) -> Vector2D {
        Vector2D {
            x: self.x / _rhs,
            y: self.y / _rhs,
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, _rhs: Vector2D) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, _rhs: Vector2D) -> Vector2D{
        Vector2D {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}