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

pub struct Points {
    pub points: Vec<Point>,
}

impl Points {
    pub fn new(num: i32) -> Self {
        let mut points: Vec<Point> = Vec::new();
        for _ in 0..num {
            points.push(Point::new());
        }

        Points { points }
    }

    pub fn add_point(&mut self) {
        self.points.push(Point::new());
    }

    pub fn remove_point(&mut self) {
        self.points.pop();
    }
}
