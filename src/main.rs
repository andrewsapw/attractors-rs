use macroquad::prelude::*;

mod point;
use point::Point;


fn step_lorenz(p: &mut Point, screen_width: f32, screen_height: f32) {
    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;

    let dt = 0.01;

    let dx = sigma * (p.coords.y - p.coords.x) * dt;
    let dy = (p.coords.x * (rho - p.coords.z) - p.coords.y) * dt;
    let dz = (p.coords.x * p.coords.y - beta * p.coords.z) * dt;

    p.coords.x += dx;
    p.coords.y += dy;
    p.coords.z += dz;
}

fn rotate_y(p: &Point, rotation_angle: f32) -> Vec3 {
    let y_rotate_matrix = mat3(
        vec3(rotation_angle.cos(), 0.0, rotation_angle.sin()),
        vec3(0.0, 1.0, 0.0),
        vec3(-rotation_angle.sin(), 0.0, rotation_angle.cos()),
    );

    let rotated_coords = y_rotate_matrix.mul_vec3(p.coords);
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
    let mut points: Vec<point::Point> = Vec::new();
    for _ in 0..20000 {
        points.push(Point::new());
    }

    let screen_width = screen_width();
    let screen_height = screen_height();

    let screen_resolution = format!("{} {}", screen_width, screen_height);
    let mut rotation_angle: f32 = 90.0;

    let black_image = Image::gen_image_color(screen_width as u16, screen_height as u16, BLACK);
    let mut image = Image::gen_image_color(screen_width as u16, screen_height as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    const ROTATE_EVERY_N_FRAMES: u32 = 100;
    let mut frame_counter = 0;

    loop {
        // reset image
        image = black_image.clone();

        for point in &mut points {
            step_lorenz(point, screen_width, screen_height);
            if is_key_down(KeyCode::D) {
                if frame_counter % ROTATE_EVERY_N_FRAMES == 0 {
                    rotation_angle += 0.0001;
                    rotation_angle %= 360.0;

                    frame_counter = 0;
                }
                frame_counter += 1;
            }

            let rotated_coords = rotate_y(point, rotation_angle);
            let screen_coords = map_coords_to_screen(rotated_coords, screen_width, screen_height);

            // draw point on image
            image.set_pixel(screen_coords.x as u32, screen_coords.y as u32, RED);
        }

        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);

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
