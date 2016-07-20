extern crate glfw;
extern crate kiss3d;
extern crate nalgebra as na;

use na::Point3;
use glfw::WindowEvent;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::{Camera, ArcBall};

fn main() {
    let mut window = Window::new("Voxel Editor");
    window.add_cube(1.0, 1.0, 1.0).set_color(1.0, 0.0, 0.0);

    let mut camera = ArcBall::new(Point3::new(2f32, 2.0, 2.0), na::origin());

    window.set_light(Light::Absolute(na::Point3::new(0.0, 10.0, 10.0)));

    while !window.should_close() {
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

        window.render_with_camera(&mut camera);
    }
}

