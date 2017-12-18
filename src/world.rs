use particle::Particle2D;
use vector::Vector2D;

pub struct World {
    pub particles: Vec<Particle2D>,
}

impl World {
    pub fn new() -> World {
        World { particles: vec![] }
    }

    pub fn update(&mut self, _time: f64) {
        for i in 0..self.particles.len() {
            let mut accel = Vector2D { x: 0.0, y: 0.0 };
            for j in 0..self.particles.len() {
                if i == j { continue; }
                let other = &self.particles[j];
                accel += self.particles[i].accel_from(other); //acceleration value
            }
            let mut p = &mut self.particles[i];
            p.velocity += accel;
            //p.velocity = p.velocity * 0.99;
            p.position += p.velocity;
        }
    }

    pub fn add_particle(&mut self, x: f64, y: f64, size: f64) {
        self.particles.push(Particle2D::new(x, y, 0., 0., size));
    }
}
