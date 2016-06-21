#[macro_use]

extern crate glium;
extern crate image;

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

    let image = image::load(Cursor::new(&include_bytes!("images/bird.png")[..]),
                            image::PNG)
        .unwrap()
        .to_rgba();
    let image_texture = glium::texture::RawImage2d::from_raw_rgba_reversed(image.to_raw(),
                                                                           image.dimensions());

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let mut state = State { rot: -0.5 };

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.5, -0.25] };
    let vertex3 = Vertex { position: [0.0, 0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    loop {
        let (state_, d) = draw(state, &vertex_buffer, &display);
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
        vertex_buffer: &glium::VertexBuffer<Vertex>,
        display: &glium::backend::glutin_backend::GlutinFacade)
        -> (State, Result<(), glium::SwapBuffersError>) {
    use glium::Surface;

    let mut rot = state.rot;
    rot += 0.0002;

    if rot > 0.5 {
        rot = -0.5;
    }

    let state_ = State { rot: rot };

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 my_attr;

        uniform mat4 matrix;

        void main() {
            my_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;
        in vec2 my_attr;

        void main() {
            color = vec4(my_attr, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let uniforms = uniform! {
        matrix: [ [rot.cos(), rot.sin(), 0.0, 0.0],
                  [-rot.sin(), rot.cos(), 0.0, 0.0],
                  [0.0, 0.0, 1.0, 0.0],
                  [rot, 0.0, 0.0, 1.0f32],
                ]
    };

    let mut target = display.draw();
    target.clear_color(251.0 / 255.0, 34.0 / 255.0, 112.0 / 255.0, 1.0);
    target.draw(vertex_buffer,
              &indices,
              &program,
              &uniforms,
              &Default::default())
        .unwrap();

    (state_, target.finish())
}
