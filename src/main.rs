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
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();
    let mut state = State {};

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
    use std::f32::consts::PI;

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.7372549019607843, 0.2549019607843137, 0.4352941176470588);
            vec3 regular_color = vec3(0.9372549019607843, 0.4549019607843137, 0.6352941176470588);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut target = display.draw();

    let perspective = {
        let (w, h) = target.get_dimensions();
        let aspect_ratio = h as f32 / w as f32;

        let fov: f32 = PI / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [[f * aspect_ratio, 0.0, 0.0, 0.0],
         [0.0, f, 0.0, 0.0],
         [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
         [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0]]
    };

    let uniforms = uniform! {
        matrix: [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ],
        u_light: [-1.0, 0.4, 0.9f32],
        perspective: perspective
    };

    target.clear_color_and_depth((0.6509803921568628, 0.9372549019607843, 0.8823529411764706, 1.0),
                                 1.0);
    target.draw((positions, normals), indices, &program, &uniforms, &params)
        .unwrap();

    (state, target.finish())
}
