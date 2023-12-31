use macroquad::prelude::*;

struct Point {
    coords: Vec3,
}

// implement random point initialization
impl Point {
    fn new() -> Self {
        let x = rand::gen_range(-10.0, 10.0);
        let y = rand::gen_range(-10.0, 10.0);
        let z = rand::gen_range(-10.0, 10.0);

        Point {
            coords: vec3(x, y, z),
        }
    }
}

fn step_lorenz(point: &mut Point, screen_width: f32, screen_height: f32) {
    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;

    let dt = 0.01;

    let dx = sigma * (point.coords.y - point.coords.x) * dt;
    let dy = (point.coords.x * (rho - point.coords.z) - point.coords.y) * dt;
    let dz = (point.coords.x * point.coords.y - beta * point.coords.z) * dt;

    point.coords.x += dx;
    point.coords.y += dy;
    point.coords.z += dz;
}

fn rotate_y(point: &Point, rotation_angle: f32) -> Vec3 {
    let y_rotate_matrix = mat3(
        vec3(rotation_angle.cos(), 0.0, rotation_angle.sin()),
        vec3(0.0, 1.0, 0.0),
        vec3(-rotation_angle.sin(), 0.0, rotation_angle.cos()),
    );

    let rotated_coords = y_rotate_matrix.mul_vec3(point.coords);
    return rotated_coords;
}

fn map_coords_to_screen(coords: Vec3, screen_width: f32, screen_height: f32) -> Vec3 {
    let x = coords.x * 10.0 + screen_width / 2.0;
    let y = coords.y * 10.0 + screen_height / 2.0;
    let z = coords.z * 10.0;

    vec3(x, y, z)
}

#[macroquad::main("Attractors")]
async fn main() {
    let mut points: Vec<Point> = Vec::new();
    // init points
    for _ in 0..5000 {
        points.push(Point::new());
    }

    let screen_width = screen_width();
    let screen_height = screen_height();

    let screen_resolution = format!("{} {}", screen_width, screen_height);

    let mut rotation_angle: f32 = 90.0;

    loop {
        clear_background(BLACK);

        // draw_line(
        //     screen_width / 2.0,
        //     0.0,
        //     screen_width / 2.0,
        //     screen_height,
        //     1.0,
        //     WHITE,
        // );
        // draw_line(
        //     0.0,
        //     screen_height / 2.0,
        //     screen_width,
        //     screen_height / 2.0,
        //     1.0,
        //     WHITE,
        // );

        for point in &mut points {
            step_lorenz(point, screen_width, screen_height);
            if is_key_down(KeyCode::D) {
                rotation_angle += 0.00001;
                rotation_angle %= 360.0;
            }

            let rotated_coords = rotate_y(point, rotation_angle);
            let screen_coords = map_coords_to_screen(
                rotated_coords,
                screen_width,
                screen_height,
            );

            // draw points
            draw_circle(
                screen_coords.x,
                screen_coords.y,
                1.0,
                Color::new(1.0, 1.0, 1.0, 0.5),
            )
        }

        draw_text(&screen_resolution, 10.0, 10.0, 16.0, RED);
        // draw rotation angle
        draw_text(
            &format!("Rotation angle: {}", rotation_angle),
            10.0,
            30.0,
            16.0,
            RED,
        );

        // draw fps
        draw_text(
            &format!("FPS: {}", get_fps()),
            screen_width - 100.0,
            10.0,
            16.0,
            RED,
        );

        next_frame().await
    }
}
