use std::ops::Range;

use lorenz::Lorenz;
use macroquad::prelude::*;

use macroquad::hash;
use macroquad::ui::widgets::Window;
use macroquad::ui::{root_ui, Skin};

mod attractor;
mod lorenz;
mod point;

use crate::attractor::Attractor;

use point::*;

struct SimulationState {
    screen_width: f32,
    screen_height: f32,
    rotation_angle: f32,
    scale: f32,
    horizontal_offset: f32,
    vertical_offset: f32,
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
    let mut points = Points::new(100000);
    let mut lorenz_attractor = Lorenz::new(7.73, 36.13, 4.9);
    let mut simulation_state = SimulationState {
        screen_width: screen_width(),
        screen_height: screen_height(),
        rotation_angle: 90.0,
        scale: 7.0,
        horizontal_offset: screen_width() / 2.0,
        vertical_offset: screen_height() / 2.0,
    };

    let black_image = Image::gen_image_color(
        simulation_state.screen_width as u16,
        simulation_state.screen_height as u16,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );

    // this image will be used to draw the points
    let mut target_image = Image::gen_image_color(
        simulation_state.screen_width as u16,
        simulation_state.screen_height as u16,
        WHITE,
    );
    let texture = Texture2D::from_image(&target_image);

    let window_skin = {
        let window_style = root_ui()
            .style_builder()
            .background(black_image.clone())
            .text_color(WHITE)
            .color(WHITE)
            .build();

        let label_style = root_ui()
            .style_builder()
            .text_color(WHITE)
            .color(WHITE)
            .build();

        Skin {
            window_style: window_style,
            label_style: label_style.clone(),
            combobox_style: label_style.clone(),
            button_style: label_style.clone(),
            checkbox_style: label_style.clone(),
            window_titlebar_style: label_style.clone(),
            ..root_ui().default_skin()
        }
    };

    // this color will be used to draw the points and text
    let color = Color::new(127.0 / 255.0, 5.0 / 255.0, 5.0 / 255.0, 0.8);
    loop {
        simulation_state.screen_width = screen_width();
        simulation_state.screen_height = screen_height();

        // reset target image
        target_image = black_image.clone();
        handle_input(&mut points, &mut simulation_state);

        plot_points(
            &mut points,
            &lorenz_attractor,
            &simulation_state,
            &mut target_image,
        );

        texture.update(&target_image);

        draw_texture(&texture, 0., 0., WHITE);

        let mouse_coords = mouse_position();
        draw_text(
            &format!("{} {}", mouse_coords.0, mouse_coords.1),
            mouse_coords.0,
            mouse_coords.1,
            15.0,
            RED,
        );
        draw_ui(&mut lorenz_attractor, &window_skin);

        draw_info(&simulation_state, points.points.len(), color);

        next_frame().await
    }
}

fn plot_points(
    points: &mut Points,
    lorenz_attractor: &Lorenz,
    simulation_state: &SimulationState,
    image: &mut Image,
) {
    for point in &mut points.points {
        let previous_coords = point.coords;
        lorenz_attractor.step(point);

        let coords_diff = {
            let diff = point.coords - previous_coords;
            (diff.x + diff.y + diff.z).sqrt()
        };

        let particle_color =
            Color::new(255.0 / 255.0, coords_diff * 60.0 / 255.0, 5.0 / 255.0, 0.7);

        let screen_coords = {
            let rotated_coords = point.rotate_y(simulation_state.rotation_angle);
            map_coords_to_screen(rotated_coords, &simulation_state)
        };

        image.set_pixel(
            screen_coords.x as u32,
            screen_coords.y as u32,
            particle_color,
        );

        let point_mirrored_coords = -point.coords;
        let screen_rotated_coords = {
            let rotated_mirrored_coords =
                rotate_y(point_mirrored_coords, simulation_state.rotation_angle);
            map_coords_to_screen(rotated_mirrored_coords, &simulation_state)
        };

        image.set_pixel(
            screen_rotated_coords.x as u32,
            screen_rotated_coords.y as u32,
            particle_color,
        );
    }
}

fn map_coords_to_screen(coords: Vec3, simulation_state: &SimulationState) -> Vec3 {
    let x = clamp(
        coords.x * simulation_state.scale + simulation_state.horizontal_offset,
        0.0,
        simulation_state.screen_width - 1.0,
    );
    let y = clamp(
        coords.y * simulation_state.scale + simulation_state.vertical_offset,
        0.0,
        simulation_state.screen_height - 1.0,
    );
    let z = coords.z * 10.0;

    vec3(x, y, z)
}

fn handle_input(points: &mut Points, simulation_meta: &mut SimulationState) {
    let mouse_pos_diff = mouse_delta_position();
    if is_mouse_button_down(MouseButton::Left) {
        simulation_meta.rotation_angle += mouse_pos_diff.x * 4.0;
        simulation_meta.rotation_angle %= 360.0;
    }

    if is_key_down(KeyCode::Space) {
        points.add_point();
    }
    if is_key_down(KeyCode::Backspace) {
        points.remove_point();
    }
    if is_key_pressed(KeyCode::R) {
        points.reset();
    }

    if is_key_down(KeyCode::Left) {
        simulation_meta.horizontal_offset -= 2.0;
        simulation_meta.horizontal_offset = clamp(
            simulation_meta.horizontal_offset,
            0.0,
            simulation_meta.screen_width - 1.0,
        );
    }
    if is_key_down(KeyCode::Right) {
        simulation_meta.horizontal_offset += 2.0;
        simulation_meta.horizontal_offset = clamp(
            simulation_meta.horizontal_offset,
            0.0,
            simulation_meta.screen_width - 1.0,
        );
    }
    if is_key_down(KeyCode::Up) {
        simulation_meta.vertical_offset += 2.0;
        simulation_meta.vertical_offset = clamp(
            simulation_meta.vertical_offset,
            0.0,
            simulation_meta.screen_height - 1.0,
        );
    }
    if is_key_down(KeyCode::Down) {
        simulation_meta.vertical_offset -= 2.0;
        simulation_meta.vertical_offset = clamp(
            simulation_meta.vertical_offset,
            0.0,
            simulation_meta.screen_height - 1.0,
        );
    }
    if is_key_down(KeyCode::W) {
        simulation_meta.scale += 0.1;
    }
    if is_key_down(KeyCode::S) {
        simulation_meta.scale -= 0.1;
    }

    if is_key_down(KeyCode::D) {
        simulation_meta.rotation_angle += 0.01;
        simulation_meta.rotation_angle %= 360.0;
    } else if is_key_down(KeyCode::A) {
        simulation_meta.rotation_angle -= 0.01;
        simulation_meta.rotation_angle %= 360.0;
    }
}

fn draw_ui(lorenz_attractor: &mut Lorenz, window_skin: &Skin) {
    Window::new(hash!(), vec2(10., 10.), vec2(300., 100.))
        .label("Parameters")
        .close_button(false)
        .ui(&mut root_ui(), |ui| {
            ui.slider(
                hash!(),
                "Sigma",
                Range {
                    start: 1.0,
                    end: 10.0,
                },
                &mut lorenz_attractor.sigma,
            );
            ui.slider(
                hash!(),
                "Rho",
                Range {
                    start: 1.0,
                    end: 40.0,
                },
                &mut lorenz_attractor.rho,
            );
            ui.slider(
                hash!(),
                "Beta",
                Range {
                    start: 1.0,
                    end: 10.0,
                },
                &mut lorenz_attractor.beta,
            );

            ui.push_skin(&window_skin);
        });
}

fn draw_info(simulation_meta: &SimulationState, num_points: usize, color: Color) {
    // let screen_resolution = format!(
    //     "{} {}",
    //     simulation_meta.screen_width, simulation_meta.screen_height
    // );
    // let rotation_angle = format!("Rotation angle: {}", simulation_meta.rotation_angle);
    // let number_of_points = format!("Number of points: {}", num_points);
    let fps = format!("FPS: {}", get_fps());

    // draw_text(&screen_resolution, 10.0, 10.0, 16.0, color);
    // draw_text(&rotation_angle, 10.0, 30.0, 16.0, color);
    // draw_text(&number_of_points, 10.0, 50.0, 16.0, color);
    draw_text(
        &fps,
        simulation_meta.screen_width - 100.0,
        10.0,
        16.0,
        color,
    );
}

fn rotate_y(coords: Vec3, rotation_angle: f32) -> Vec3 {
    let y_rotate_matrix = mat3(
        vec3(rotation_angle.cos(), 0.0, rotation_angle.sin()),
        vec3(0.0, 1.0, 0.0),
        vec3(-rotation_angle.sin(), 0.0, rotation_angle.cos()),
    );

    let rotated_coords = y_rotate_matrix.mul_vec3(coords);
    return rotated_coords;
}
