use std::time::Instant;
use glam::Vec2;
use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, GlRequest, ContextBuilder, Api, event::{Event, WindowEvent}, ContextWrapper, PossiblyCurrent, GlProfile, dpi::{PhysicalPosition}};

use crate::{event::{EventSystem, WindowResizeEvent}, input::{Input}, lz_core_warn, time, graphics::{renderer::Renderer, window::window_listeners::WindowListeners, Window}};

use super::event_mapper;

pub struct GlutinWindow {
    render_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>,
    target_fps: u64,
    event_listeners: WindowListeners,
}

impl Window for GlutinWindow {
    fn run(self: Box<Self>, mut renderer: Renderer, mut event_system: EventSystem, mut lz_input: Input) {
        let mut next_frame_time: u128 = 0;

        // Move all properties of self in to their own variables, because self will get moved by event_loop.run and its 
        // can not be used in the function of event_loop.run
        let render_context = self.render_context;
        let mut event_listeners = self.event_listeners;
        let target_fps = self.target_fps;
        let event_loop = self.event_loop;

        // Read events from Scene::new
        Self::read_event_listeners(&mut event_listeners, &render_context.window());

        event_loop.run(move |event, _, control_flow| {
            let start_time = Instant::now();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        render_context.resize(physical_size);

                        event_system.send(WindowResizeEvent {
                            width: physical_size.width, 
                            height: physical_size.height 
                        })
                    },
                    WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                        if let Some(key) = input.virtual_keycode {
                            lz_input.register_key_event(
                                event_mapper::map_glutin_keycode(key), 
                                event_mapper::map_glutin_key_state(input.state)
                            );
                        }
                    },
                    WindowEvent::MouseInput { device_id: _, state, button, .. } => {
                        lz_input.register_mouse_button_event(
                            event_mapper::map_glutin_mouse_button(button), 
                            event_mapper::map_glutin_mouse_button_state(state)
                        );
                    },
                    WindowEvent::MouseWheel { device_id: _, delta, phase: _, .. } => {
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
                    WindowEvent::CursorMoved { device_id: _, position, .. } => {
                        lz_input.register_mouse_reposition_event(position.x, position.y);
                    },
                    _ => (),
                },
                Event::DeviceEvent { device_id: _, event } => match event {
                    glutin::event::DeviceEvent::MouseMotion { delta } => {
                        lz_input.register_mouse_move_event(delta.0, delta.1);
                    },
                    _ => (),
                }
                _ => ()
            }

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    if time::now_millis() < next_frame_time {
                        *control_flow = ControlFlow::Poll;
                        return;
                    }

                    renderer.scene.update(&mut event_system, &lz_input);
                    Self::read_event_listeners(&mut event_listeners, &render_context.window());

                    renderer.draw();
                    render_context.swap_buffers().expect("Failed to swap buffers");

                    lz_input.reset();

                    next_frame_time = get_next_frame_time(start_time, target_fps);
                    *control_flow = ControlFlow::Poll;
                }
            }
        });
    }

    fn get_size(&self) -> Vec2 {
        return Vec2 { 
            x: self.render_context.window().inner_size().width as f32, 
            y: self.render_context.window().inner_size().height as f32,
        }
    }
}

impl GlutinWindow {
    pub fn new(name: String, event_system: &mut EventSystem) -> Self {
        let window = WindowBuilder::new()
            .with_title(name)
            .with_inner_size(glutin::dpi::PhysicalSize::new(1600, 1200));

        let event_loop = EventLoop::new();

        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .build_windowed(window, &event_loop)
            .expect("Cannot create window context");

        let render_context = unsafe {
            gl_context
                .make_current()
                .expect("Failed to make context current")
        };

        gl::load_with(|ptr| render_context.get_proc_address(ptr) as *const _);

        return Self {
            render_context,
            event_loop,
            target_fps: 60,
            event_listeners: WindowListeners::new(event_system),
        }
    }

    fn lock_cursor(window: &glutin::window::Window) {
        match window.set_cursor_grab(glutin::window::CursorGrabMode::Locked) {
            Ok(_) => (),
            Err(err) => {
                lz_core_warn!("could not lock cursor: {}",  err.to_string());
            },
        }
    }

    fn unlock_cursor(window: &glutin::window::Window) {
        match window.set_cursor_grab(glutin::window::CursorGrabMode::None) {
            Ok(_) => (),
            Err(err) => {
                lz_core_warn!("could not unlock cursor: {}",  err.to_string());
            },
        }
    }

    /// Confine the cursor the the window area
    fn confine_cursor(window: &glutin::window::Window) {
        match window.set_cursor_grab(glutin::window::CursorGrabMode::Confined) {
            Ok(_) => (),
            Err(err) => {
                lz_core_warn!("could not confine cursor: {}",  err.to_string());
            },
        }
    }

    fn hide_cursor(window: &glutin::window::Window) {
        window.set_cursor_visible(false);
    }

    fn show_cursor(window: &glutin::window::Window) {
        window.set_cursor_visible(true);
    }

    fn set_cursor_position(window: &glutin::window::Window, x: f32, y: f32) {
        match window.set_cursor_position(PhysicalPosition{x, y}) {
            Ok(_) => (),
            Err(err) => {
                lz_core_warn!("could not set cursor position: {}",  err.to_string());
            },
        }
    }

    fn read_event_listeners(event_listeners: &mut WindowListeners, window: &glutin::window::Window) {
        if event_listeners.lock_cursor_listener.read().len() > 0 { Self::lock_cursor(window) }
        if event_listeners.unlock_cursor_listener.read().len() > 0 { Self::unlock_cursor(window) }
        if event_listeners.confine_cursor_listener.read().len() > 0 { Self::confine_cursor(window) }
        if event_listeners.hide_cursor_listener.read().len() > 0 { Self::hide_cursor(window) }
        if event_listeners.show_cursor_listener.read().len() > 0 { Self::show_cursor(window) }
        if let Some(event) = event_listeners.set_cursor_position_listener.read().last() {
            Self::set_cursor_position(window, event.x, event.y);
        }
    }
}

fn get_next_frame_time(start_time: Instant, target_fps: u64) -> u128 {
    let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u128;
    let frame_time_ms = 1000 / target_fps as u128;
    return time::now_millis() + frame_time_ms - elapsed_time;
}
