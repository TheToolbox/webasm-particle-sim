#[macro_use]
extern crate lazy_static;
mod vector;
mod particle;
mod world;
use self::world::World;
use std::sync::Mutex;


lazy_static! {
    static ref W : Mutex<World> = Mutex::new(World::new());
}

#[no_mangle]
pub fn update(time: f64) {
    W.lock().unwrap().update(time);
}

#[no_mangle]
pub fn add_particle(x: f64, y: f64, size: f64) {
    W.lock().unwrap().add_particle(x, y, size);
}

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_particle(_: f64, _: f64, _: f64, _: f64);
}

#[no_mangle]
pub extern "C" fn draw() {
    unsafe {
        clear_screen();
    }
    let particles = &W.lock().unwrap().particles;
    for p in particles {
        unsafe {
            draw_particle(p.position.x, p.position.y, p.mass, p.velocity.magnitude());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
