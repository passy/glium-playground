#[macro_use]

extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

struct State {
    rot: f32,
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let mut state = State { rot: -0.5 };

    loop {
        let (state_, d) = draw(state, &display);
        state = state_;
        d.unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}

fn draw(state: State,
        display: &glium::backend::glutin_backend::GlutinFacade)
        -> (State, Result<(), glium::SwapBuffersError>) {
    use glium::Surface;

    let mut rot = state.rot;
    rot += 0.0002;

    if rot > 0.5 {
        rot = -0.5;
    }

    let state_ = State { rot: rot };

    let vertex1 = Vertex { position: [-0.5 + rot, -0.5] };
    let vertex2 = Vertex { position: [0.5 + rot, -0.25] };
    let vertex3 = Vertex { position: [0.0 + rot, 0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut target = display.draw();
    target.clear_color(251.0 / 255.0, 34.0 / 255.0, 112.0 / 255.0, 1.0);
    target.draw(&vertex_buffer,
              &indices,
              &program,
              &glium::uniforms::EmptyUniforms,
              &Default::default())
        .unwrap();

    (state_, target.finish())
}
