#[macro_use]

extern crate glium;
extern crate image;

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

    let image = image::load(Cursor::new(&include_bytes!("../images/bird.png")[..]),
                            image::PNG)
        .unwrap()
        .to_rgba();
    let image_dimens = image.dimensions();
    let image_texture = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                           image_dimens);

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let mut state = State { rot: -0.5 };

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.5, -0.25],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [1.0, 0.0],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let texture = glium::texture::Texture2d::new(&display, image_texture).unwrap();

    loop {
        let (state_, d) = draw(state, &vertex_buffer, &display, &texture);
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
        vertex_buffer: &glium::VertexBuffer<Vertex>,
        display: &glium::backend::glutin_backend::GlutinFacade,
        texture: &glium::texture::Texture2d)
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
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ rot , 0.0, 0.0, 1.0f32],
        ],
        tex: texture
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
