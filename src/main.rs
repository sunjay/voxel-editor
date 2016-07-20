extern crate glfw;
extern crate kiss3d;
extern crate nalgebra as na;

use na::Point3;
use glfw::{WindowEvent, Key, Action};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::{Camera, ArcBall};

fn main() {
    let mut window = Window::new("Voxel Editor");
    window.add_cube(1.0, 1.0, 1.0).set_color(1.0, 0.0, 0.0);

    let mut camera = ArcBall::new(Point3::new(2f32, 2.0, 2.0), na::origin());

    window.set_light(Light::Absolute(na::Point3::new(0.0, 10.0, 10.0)));

    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;

    while !window.should_close() {
        {
            let gwindow = window.glfw_window();
            ctrl = gwindow.get_key(Key::LeftControl) == Action::Press || gwindow.get_key(Key::RightControl) == Action::Press;
            alt = gwindow.get_key(Key::LeftAlt) == Action::Press || gwindow.get_key(Key::RightAlt) == Action::Press;
            shift = gwindow.get_key(Key::LeftShift) == Action::Press || gwindow.get_key(Key::RightShift) == Action::Press;
        }

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Scroll(x, y) => {
                    let event = WindowEvent::Scroll(x, -y);
                    camera.handle_event(&window.glfw_window(), &event);
                },
                _ => {}
            }
            // do not let any events get handled by the engine
            event.inhibited = true;
        }

        println!("ctrl: {}, alt: {}, shift: {}", ctrl, alt, shift);

        window.render_with_camera(&mut camera);
    }
}

