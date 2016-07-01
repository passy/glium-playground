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

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();
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

        out vec3 v_normal;

        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let uniforms = uniform! {
        matrix: [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        u_light: [-1.0, 0.4, 0.9f32]
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
    target.draw((positions, normals), indices, &program, &uniforms, &params)
        .unwrap();

    (state, target.finish())
}
