#[macro_use]
extern crate glium;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::glutin;

    let vertex_shader_src = r#"
        #version 140
        
        in vec2 position;
        uniform float t;
        
        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
        "#;

    let fragment_shader_src = r#"
        #version 140
        
        out vec4 color;
        
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
        "#;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    let vert1 = Vertex {
        position: [-0.8, -0.7],
    };
    let vert2 = Vertex {
        position: [0.4, 0.1],
    };
    let vert3 = Vertex {
        position: [0.0, 0.8],
    };
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let shape = vec![vert1, vert2, vert3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut t: f32 = -0.5;
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.3, 0.4, 0.75, 1.0);

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! { t: t },
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();
    });
}
