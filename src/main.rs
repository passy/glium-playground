#[macro_use]

extern crate glium;

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(251.0 / 255.0, 34.0 / 255.0, 112.0 / 255.0, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
