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

    pub fn rotate_y(&self, rotation_angle: f32) -> Vec3 {
        let y_rotate_matrix = mat3(
            vec3(rotation_angle.cos(), 0.0, rotation_angle.sin()),
            vec3(0.0, 1.0, 0.0),
            vec3(-rotation_angle.sin(), 0.0, rotation_angle.cos()),
        );

        let rotated_coords = y_rotate_matrix.mul_vec3(self.coords);
        return rotated_coords;
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

    pub fn reset(&mut self) {
        for idx in 0..self.points.len() {
            self.points[idx] = Point::new();
        }
    }

    pub fn add_point(&mut self) {
        self.points.push(Point::new());
    }

    pub fn remove_point(&mut self) {
        self.points.pop();
    }
}
