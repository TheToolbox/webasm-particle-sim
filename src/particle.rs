use vector::Vector2D;

#[derive(Clone, Copy, Default)]
pub struct Particle2D {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub mass: f64,
}

impl Particle2D {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Particle2D {
        Particle2D {
            position: Vector2D { x: x, y: y },
            velocity: Vector2D { x: vx, y: vy },
            mass: mass,
        }
    }

    pub fn accel_from(&self, other: &Particle2D) -> Vector2D {
        let dir = other.position - self.position;
        let r = dir.magnitude();
        if r < self.mass + other.mass {
            Vector2D { x: 0., y: 0. }
        } else {
            dir * other.mass / (r * r * r)
        }
    }

    /*pub fn random(mut rng: &mut Rng, range: &Range<f64>) -> Particle2D {
        Particle2D {
            position: Vector2D::random(&mut rng, range),
            velocity: Vector2D::random(&mut rng, range),
            mass: range.ind_sample(&mut rng),
        }
    }*/
}
