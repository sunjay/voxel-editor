extern crate glfw;
extern crate kiss3d;
extern crate nalgebra as na;

use glfw::WindowEvent;
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Voxel Editor");
    let mut c = window.add_cube(0.5, 0.5, 0.5);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    while !window.should_close() {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Scroll(x, y) => println!("{} {}", x, y),
                _ => {}
            }
            // do not let any events get handled by the engine
            event.inhibited = true;
        }

        window.render();
    }
}

