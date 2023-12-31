use macroquad::prelude::*;

mod point;
use point::*;

struct SimulationMeta {
    screen_width: f32,
    screen_height: f32,
    rotation_angle: f32,
}

fn step_lorenz(p: &mut Point) {
    let sigma = 4.0;
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
    let mut points = Points::new(100000);

    let screen_width = screen_width();
    let screen_height = screen_height();

    let rotation_angle: f32 = 90.0;
    let mut simulation_meta = SimulationMeta {
        screen_width,
        screen_height,
        rotation_angle,
    };

    let black_image = Image::gen_image_color(screen_width as u16, screen_height as u16, BLACK);
    let mut image = Image::gen_image_color(screen_width as u16, screen_height as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    loop {
        image = black_image.clone();

        handle_input(&mut points, &mut simulation_meta);
        for point in &mut points.points {
            step_lorenz(point);

            let rotated_coords = rotate_y(point, simulation_meta.rotation_angle);
            let screen_coords = map_coords_to_screen(rotated_coords, screen_width, screen_height);

            image.set_pixel(screen_coords.x as u32, screen_coords.y as u32, RED);
        }

        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);

        draw_info(&simulation_meta, points.points.len());
        next_frame().await
    }
}

fn handle_input(points: &mut Points, simulation_meta: &mut SimulationMeta) {
    if is_key_down(KeyCode::Space) {
        points.add_point();
    }
    if is_key_down(KeyCode::Backspace) {
        points.remove_point();
    }
    if is_key_pressed(KeyCode::R) {
        points.reset();
    }


    if is_key_down(KeyCode::D) {
        simulation_meta.rotation_angle += 0.01;
        simulation_meta.rotation_angle %= 360.0;
    } else if is_key_down(KeyCode::A) {
        simulation_meta.rotation_angle -= 0.01;
        simulation_meta.rotation_angle %= 360.0;
    }
}

fn draw_info(simulation_meta: &SimulationMeta, num_points: usize) {
    let screen_resolution = format!(
        "{} {}",
        simulation_meta.screen_width, simulation_meta.screen_height
    );
    let rotation_angle = format!("Rotation angle: {}", simulation_meta.rotation_angle);
    let number_of_points = format!("Number of points: {}", num_points);
    let fps = format!("FPS: {}", get_fps());

    draw_text(&screen_resolution, 10.0, 10.0, 16.0, RED);
    draw_text(&rotation_angle, 10.0, 30.0, 16.0, RED);
    draw_text(&number_of_points, 10.0, 50.0, 16.0, RED);
    draw_text(&fps, simulation_meta.screen_width - 100.0, 10.0, 16.0, RED);
}
