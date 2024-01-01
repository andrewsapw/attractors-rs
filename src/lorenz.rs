// import attractor trait
use crate::attractor::Attractor;
use crate::point::Point;

pub struct Lorenz {
    // Define the parameters of the Lorenz attractor
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl Lorenz {
    pub fn new(sigma: f32, rho: f32, beta: f32) -> Self {
        Lorenz { sigma, rho, beta }
    }
}

impl Attractor for Lorenz {
    fn step(&self, p: &mut Point) {
        let dt = 0.01;

        let dx = self.sigma * (p.coords.y - p.coords.x) * dt;
        let dy = (p.coords.x * (self.rho - p.coords.z) - p.coords.y) * dt;
        let dz = (p.coords.x * p.coords.y - self.beta * p.coords.z) * dt;

        p.coords.x += dx;
        p.coords.y += dy;
        p.coords.z += dz;
    }
}
