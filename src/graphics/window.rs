use std::time::{Instant, SystemTime, UNIX_EPOCH};
use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, GlRequest, ContextBuilder, Api, event::{Event, WindowEvent}, ContextWrapper, PossiblyCurrent, GlProfile};

use crate::{event::{EventSystem, WindowResizeEvent}, lz_core_info, lz_core_err};

use super::renderer::Renderer;

pub struct Window {
    render_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>,
    target_fps: u64,
    wireframe_mode: bool,
}

impl Window {
    pub fn new(name: String) -> Self{
        let window = WindowBuilder::new()
            .with_title(name)
            .with_inner_size(glutin::dpi::LogicalSize::new(800, 600));

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
    
    pub fn run(self, mut renderer: Renderer, mut event_system: EventSystem) {
        let mut next_frame_time: u128 = 0;

        self.event_loop.run(move |event, _, control_flow| {
            let start_time = Instant::now();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        self.render_context.resize(physical_size);

                        event_system.send(WindowResizeEvent {
                            width: physical_size.width, 
                            height: physical_size.height 
                        })
                    },
                    WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                        match input.state {
                            glutin::event::ElementState::Pressed => {
                                if let Some(keycode) = input.virtual_keycode {
                                    // TODO send event
                                    lz_core_info!("pressed: {:?} - {:?}", keycode, input.scancode);
                                }
                            },
                            glutin::event::ElementState::Released => {
                                if let Some(keycode) = input.virtual_keycode {
                                    // TODO send event
                                    lz_core_info!("released: {:?} - {:?}", keycode, input.scancode);
                                }
                            },
                        }
                    },
                    WindowEvent::MouseInput { device_id: _, state, button, modifiers: _ } => {
                        // TODO send event. Convert state and button to our own id/enum.
                        lz_core_info!("button {:?} to state {:?}", button, state);
                    },
                    WindowEvent::MouseWheel { device_id: _, delta, phase, modifiers: _ } => {
                        match delta {
                            glutin::event::MouseScrollDelta::LineDelta(_, _) => {
                                // TODO
                                lz_core_err!("Unhandled delta type of MouseWheel event: {:?}", delta);
                            },
                            glutin::event::MouseScrollDelta::PixelDelta(movement) => {
                                // TODO send event of movement.x and movement.y and phase
                            },
                        }
                    },
                    WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                        // TODO send event
                        // lz_core_info!("mouse moved: {} / {}", position.x, position.y);
                    },
                    _ => (),
                },
                _ => ()
            }

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() < next_frame_time {
                        *control_flow = ControlFlow::Poll;
                        return;
                    }

                    renderer.scene.update(&mut event_system);
                    renderer.draw();
                    self.render_context.swap_buffers().expect("Failed to swap buffers");
                    self.render_context.window().request_redraw();

                    next_frame_time = get_next_frame_time(start_time, self.target_fps);
                    *control_flow = ControlFlow::Poll;
                }
            }
        });
    }
}

fn get_next_frame_time(start_time: Instant, target_fps: u64) -> u128 {
    let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u128;
    let frame_time_ms = 1000 / target_fps as u128;
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + frame_time_ms - elapsed_time;
}