use std::time::{Instant, SystemTime, UNIX_EPOCH};
use glam::Vec2;
use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, GlRequest, ContextBuilder, Api, event::{Event, WindowEvent}, ContextWrapper, PossiblyCurrent, GlProfile};

use crate::{event::{EventSystem, WindowResizeEvent}, input::{Input, glutin_mapper}, lz_core_warn};

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
            .with_inner_size(glutin::dpi::PhysicalSize::new(1600, 1200));

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

    pub fn get_size(&self) -> Vec2 {
        return Vec2 { 
            x: self.render_context.window().inner_size().width as f32, 
            y: self.render_context.window().inner_size().height as f32,
        }
    }
    
    pub fn run(self, mut renderer: Renderer, mut event_system: EventSystem, mut lz_input: Input) {
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
                        if let Some(key) = input.virtual_keycode {
                            lz_input.register_key_event(
                                glutin_mapper::map_glutin_keycode(key), 
                                glutin_mapper::map_glutin_key_state(input.state)
                            );
                        }
                    },
                    WindowEvent::MouseInput { device_id: _, state, button, modifiers: _ } => {
                        lz_input.register_mouse_button_event(
                            glutin_mapper::map_glutin_mouse_button(button), 
                            glutin_mapper::map_glutin_mouse_button_state(state)
                        );
                    },
                    WindowEvent::MouseWheel { device_id: _, delta, phase: _, modifiers: _ } => {
                        match delta {
                            glutin::event::MouseScrollDelta::LineDelta(x, y) => {
                                // TODO how is this triggered? Can we register it the same as the other type?
                                lz_core_warn!("Unchecked MouseWheel event delta type: {:?}", delta);
                                lz_input.register_scroll_x_event(x as f64);
                                lz_input.register_scroll_y_event(y as f64);
                            },
                            glutin::event::MouseScrollDelta::PixelDelta(movement) => {
                                lz_input.register_scroll_x_event(movement.x);
                                lz_input.register_scroll_y_event(movement.y);
                            },
                        }
                    },
                    WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                        lz_input.register_mouse_move_x_event(position.x);
                        lz_input.register_mouse_move_y_event(position.y);
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

                    renderer.scene.update(&mut event_system, &lz_input);
                    renderer.draw();
                    self.render_context.swap_buffers().expect("Failed to swap buffers");
                    self.render_context.window().request_redraw();

                    lz_input.reset();

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
