#[macro_use]

extern crate glium;
extern crate image;
mod teapot;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

struct State {
    rot: f32,
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    use glium::DisplayBuild;
    use std::io::Cursor;

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let mut state = State { rot: -0.5 };

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES)
        .unwrap();

    loop {
        let (state_, d) = draw(state, &display, &positions, &normals, &indices);
        state = state_;
        d.unwrap();

        for ev in display.poll_events() {
            if let glium::glutin::Event::Closed = ev {
                return;
            }
        }
    }
}

fn draw(state: State,
        display: &glium::backend::glutin_backend::GlutinFacade,
        positions: &glium::VertexBuffer<teapot::Vertex>,
        normals: &glium::VertexBuffer<teapot::Normal>,
        indices: &glium::IndexBuffer<u16>)
        -> (State, Result<(), glium::SwapBuffersError>) {
    use glium::Surface;

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
    };

    let mut target = display.draw();
    target.clear_color(251.0 / 255.0, 34.0 / 255.0, 112.0 / 255.0, 1.0);
    target.draw((positions, normals),
              indices,
              &program,
              &uniforms,
              &Default::default())
        .unwrap();

    (state, target.finish())
}
