#[macro_use]

extern crate glium;

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
}
