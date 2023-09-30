extern crate gl;

use std::f32::consts::PI;
use std::time::Instant;
use glium::glutin::{Api, ContextBuilder, GlRequest};
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface, VertexBuffer};
use glium::index::{ PrimitiveType, IndexBuffer };

use nalgebra as na;
use core::Vertex;

pub fn main() {
    let event_loop = EventLoop::new();
    let start_time = Instant::now();

    let window = WindowBuilder::new()
        .with_title("Heaven");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let display = Display::from_gl_window(gl_context)
        .expect("Cannot get the Display from OpenGL context");

    let program = glium::Program::from_source(
        &display,
        include_str!("../../shaders/vertex.glsl"),
        include_str!("../../shaders/fragment.glsl"),
        None
    ).expect("Cannot crate the program from the display and shaders");



    let shape = vec![
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0,  1.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, 0.0,  1.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0,  1.0] },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0,  1.0] },
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2,
        2, 3, 0,
        4, 5, 6,
        6, 7, 4,
        0, 4, 7,
        7, 3, 0,
        1, 5, 6,
        6, 2, 1,
        0, 4, 5,
        5, 1, 0,
        3, 7, 6,
        6, 2, 3,
    ];

    let buffer_test = VertexBuffer::persistent(
        &display,
        shape.as_slice()
    ).unwrap();

    let indices = IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &indices
    ).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => display.gl_window().resize(physical_size),
                _ => ()
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }


                let t = (std::time::Instant::now() - start_time).as_secs_f32();
                let model_rotation = na::Rotation3::from_euler_angles(0.0, t as f32, 0.0).to_homogeneous();
                let model = model_rotation * na::Matrix4::identity();

                // View transform.
                let eye = na::Point3::new(0.0f32, 0.0, 3.0);
                let target = na::Point3::new(0.0, 0.0, 0.0);
                let up = na::Vector3::new(0.0, 1.0, 0.0);
                let view = na::Isometry3::look_at_rh(&eye, &target, &up).to_homogeneous();

                // Perspective projection.
                let projection = *na::Perspective3::new(
                    1.0,
                    PI / 2.0,
                    0.1,
                    100.0
                ).as_matrix();

                let m = projection * view * model;
                let m_array = *m.as_ref();

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                target.draw(
                    &buffer_test,
                    &indices,
                    &program,
                    &glium::uniform! { m: m_array },
                    &Default::default()
                )
                    .unwrap();
                target.finish().unwrap();
            }
            _ => (),
        }
    });
}
