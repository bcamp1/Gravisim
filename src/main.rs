extern crate sdl2;

mod body;
mod system;
mod cam;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::KeyboardState;
use sdl2::mouse::MouseState;
use sdl2::keyboard::Scancode;
use sdl2::event::{Event, WindowEvent};
use sdl2::gfx::primitives::DrawRenderer;

const GRAVITY_CONST: f32 = 0.0005;
const PI: f32 = 3.14159265;

fn main() {
    let mut cam = cam::Cam::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window_size = (1280, 720);
    let mut draw_size = (0, 0);

    let mut res_mult = 1.0;

    let window = video_subsystem.window(format!("Gravisim").as_str(), window_size.0, window_size.1)
        .position_centered()
        //.allow_highdpi()
        .build()
        .unwrap();

    draw_size = window.drawable_size();
    res_mult = draw_size.0 as f32 / window.size().0 as f32;

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut system = system::System::new();

    // Editor
    let mut selected_size: f32 = 50.0;
    let mut selected_pos = (0f32, 0f32);
    let mut selected_vel = (0f32, 0f32);
    let mut pos_selected = false;

    let mut mouse_x_raw = 0f32;
    let mut mouse_y_raw = 0f32;
    let mut mouse_x = 0f32;
    let mut mouse_y = 0f32;
    let mut raw_zoom = 0f32;

    let mut mouse_pressed = false;
    let mut pmouse_pressed = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::C), ..} => {
                    system.bodies = vec!();
                },
                Event::MouseWheel {y: y_pos, ..} => {
                    let delta_zoom = 0.01 * y_pos as f32;
                    let focus_point = cam.reverse_transform((mouse_x, mouse_y));
                    cam.zoom += delta_zoom;
                    cam.x += delta_zoom * focus_point.0;
                    cam.y += delta_zoom * focus_point.1;
                },
                _ => {}
            }
        }

        let key_state = KeyboardState::new(&event_pump);
        let mouse_state = MouseState::new(&event_pump);


        mouse_x= (mouse_state.x() as f32 * res_mult);
        mouse_y = (mouse_state.y() as f32 * res_mult);

        if !pos_selected {
            selected_pos = cam.reverse_transform((mouse_x, mouse_y));
        }

        pmouse_pressed = mouse_pressed;
        mouse_pressed = mouse_state.left();

        if mouse_pressed && !pmouse_pressed && !pos_selected {
            pos_selected = true;
            pmouse_pressed = true;
        }

        if pos_selected {
            if mouse_pressed && pmouse_pressed {
                let point1 = selected_pos;
                let point2 = cam.reverse_transform((mouse_x, mouse_y));
                selected_vel = ((point2.0 - point1.0) / 50.0, (point2.1 - point1.1) / 50.0);
            } else {
                pos_selected = false;
                system.add(selected_pos.0, selected_pos.1, selected_vel.0, selected_vel.1, 1.0, selected_size / cam.zoom);
            }
        }

        // Pan and zoom
        if key_state.is_scancode_pressed(Scancode::D) {
            cam.x += 1.0 / cam.zoom;
        }
        if key_state.is_scancode_pressed(Scancode::A) {
            cam.x -= 1.0 / cam.zoom;
        }
        if key_state.is_scancode_pressed(Scancode::W) {
            cam.y -= 1.0 / cam.zoom;
        }
        if key_state.is_scancode_pressed(Scancode::S) {
            cam.y += 1.0 / cam.zoom;
        }
        if key_state.is_scancode_pressed(Scancode::Z) {
            selected_size += 0.1;
            if selected_size < 1.0 {
                selected_size = 1.0;
            }
        }
        if key_state.is_scancode_pressed(Scancode::X) {
            selected_size -= 0.1;
            if selected_size < 1.0 {
                selected_size = 1.0;
            }
        }

        system.update();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let selected_transformed = cam.transform(selected_pos);
        canvas.filled_circle(selected_transformed.0 as i16, selected_transformed.1 as i16, (selected_size) as i16, (255, 255, 255, 50));

        if pos_selected {
            let selected_vel_transformed = cam.transform(selected_vel);
            let point1 = selected_transformed;
            let point2 = (mouse_x, mouse_y);
            canvas.thick_line(point1.0 as i16, point1.1 as i16, point2.0 as i16, point2.1 as i16, 5, (255, 255, 255, 50));
        }

        system.render(&mut canvas, &cam);
        canvas.present();
    }
}
