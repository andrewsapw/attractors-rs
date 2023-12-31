use lorenz::Lorenz;
use macroquad::prelude::*;

mod attractor;
mod lorenz;
mod point;

use crate::attractor::Attractor;

use point::*;

struct SimulationState {
    screen_width: f32,
    screen_height: f32,
    rotation_angle: f32,
}

fn config() -> Conf {
    Conf {
        window_title: "test".to_string(),
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let mut points = Points::new(50000);
    set_fullscreen(false);
    request_new_screen_size(600.0, 600.0);

    let lorenz_attractor = Lorenz::new(4.0, 28.0, 8.0 / 3.0);

    let rotation_angle: f32 = 90.0;
    let mut simulation_state = SimulationState {
        screen_width: screen_width(),
        screen_height: screen_height(),
        rotation_angle: rotation_angle,
    };

    // this black image will be used to clear the screen
    let black_image = Image::gen_image_color(
        simulation_state.screen_width as u16,
        simulation_state.screen_height as u16,
        BLACK,
    );

    // this image will be used to draw the points
    let mut image = Image::gen_image_color(
        simulation_state.screen_width as u16,
        simulation_state.screen_height as u16,
        WHITE,
    );
    let texture = Texture2D::from_image(&image);
    
    // this color will be used to draw the points and text 
    let color = Color::new(255.0 / 255.0, 5.0 / 255.0, 5.0 / 255.0, 0.8);

    loop {
        simulation_state.screen_width = screen_width();
        simulation_state.screen_height = screen_height();

        image = black_image.clone();

        handle_input(&mut points, &mut simulation_state);
        for point in &mut points.points {
            lorenz_attractor.step(point);

            let rotated_coords = point.rotate_y(simulation_state.rotation_angle);
            let screen_coords = map_coords_to_screen(
                rotated_coords,
                simulation_state.screen_width,
                simulation_state.screen_height,
            );

            image.set_pixel(screen_coords.x as u32, screen_coords.y as u32, color);
        }

        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);

        draw_info(&simulation_state, points.points.len(), color);
        next_frame().await
    }
}



fn map_coords_to_screen(coords: Vec3, screen_width: f32, screen_height: f32) -> Vec3 {
    let x = clamp(coords.x * 5.0 + screen_width / 2.0, 0.0, screen_width - 1.0);
    let y = clamp(
        coords.y * 5.0 + screen_height / 2.0,
        0.0,
        screen_height - 1.0,
    );
    let z = coords.z * 10.0;

    vec3(x, y, z)
}

fn handle_input(points: &mut Points, simulation_meta: &mut SimulationState) {
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

fn draw_info(simulation_meta: &SimulationState, num_points: usize, color: Color) {
    let screen_resolution = format!(
        "{} {}",
        simulation_meta.screen_width, simulation_meta.screen_height
    );
    let rotation_angle = format!("Rotation angle: {}", simulation_meta.rotation_angle);
    let number_of_points = format!("Number of points: {}", num_points);
    let fps = format!("FPS: {}", get_fps());

    draw_text(&screen_resolution, 10.0, 10.0, 16.0, color);
    draw_text(&rotation_angle, 10.0, 30.0, 16.0, color);
    draw_text(&number_of_points, 10.0, 50.0, 16.0, color);
    draw_text(
        &fps,
        simulation_meta.screen_width - 100.0,
        10.0,
        16.0,
        color,
    );
}
