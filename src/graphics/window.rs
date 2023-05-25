use std::time::Instant;

use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, GlRequest, ContextBuilder, Api, event::{Event, WindowEvent}, ContextWrapper, PossiblyCurrent, GlProfile};

use super::renderer::Renderer;

pub struct Window {
    render_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>,
    target_fps: u64,
    wireframe_mode: bool,
}

impl Window {
    pub fn new(name: String) -> Self{
        let window = WindowBuilder::new().with_title(name);
        let event_loop = EventLoop::new();

        let  gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .build_windowed(window, &event_loop)
            .expect("Cannot create window context");

        let gl_context = unsafe {
            gl_context
                .make_current()
                .expect("Failed to make context current")
        };

        gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        return Self {
            render_context: gl_context,
            event_loop,
            target_fps: 60,
            wireframe_mode: false,
        }
    }
    
    pub fn run(self, mut renderer: Renderer) {
        self.event_loop.run(move |event, _, control_flow| {
            
            if self.wireframe_mode {
                unsafe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                }
            }

            let start_time = Instant::now();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => self.render_context.resize(physical_size),
                    _ => (),
                },
                _ => ()
            }

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    renderer.draw();
                    self.render_context.swap_buffers().expect("Failed to swap buffers");
                    self.render_context.window().request_redraw();

                    *control_flow = wait_until_next_frame(start_time, self.target_fps);
                }
            }
        });
    }
}

fn wait_until_next_frame(start_time: Instant, target_fps: u64) -> ControlFlow {
    let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u64;
    let frame_time_ms = 1000 / target_fps;

    if elapsed_time < frame_time_ms {
        std::thread::sleep(std::time::Duration::from_millis(frame_time_ms - elapsed_time));
    }
    
    return  ControlFlow::WaitUntil(Instant::now() + std::time::Duration::from_millis(frame_time_ms));
}
