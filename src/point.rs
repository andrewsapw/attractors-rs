use macroquad::prelude::*;

pub struct Point {
    pub coords: Vec3,
}

impl Point {
    pub fn new() -> Self {
        let x = rand::gen_range(-10.0, 10.0);
        let y = rand::gen_range(-10.0, 10.0);
        let z = rand::gen_range(-10.0, 10.0);

        Point {
            coords: vec3(x, y, z),
        }
    }
}