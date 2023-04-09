use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, GlRequest, ContextBuilder, Api, event::{Event, WindowEvent}};
use crate::renderer;

pub fn run(name: String) {
    let window = WindowBuilder::new().with_title(name);
    let event_loop = EventLoop::new();

    let  gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create window context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);


    let renderer = renderer::Renderer::new().expect("Can not create renderer");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.draw();
                gl_context.swap_buffers().expect("Failed to swap buffers")
            }
            _ => ()
        }
    });
}  
