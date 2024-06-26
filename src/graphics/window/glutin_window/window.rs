use std::time::Instant;
use glam::Vec2;
use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, GlRequest, ContextBuilder, Api, event::{Event, WindowEvent}, ContextWrapper, PossiblyCurrent, GlProfile, dpi::{PhysicalPosition, LogicalSize, LogicalPosition}};

use crate::{event::{EventSystem, WindowResizeEvent, PixelDensityChangeEvent}, input::Input, time, graphics::{renderer::Renderer, window::window_listeners::WindowListeners, Window, ui::Interface}, asset_manager::AssetManager, log::{self}};

use super::event_mapper;

/**
 * !! ISSUES !!
 * 
 * 1. On startup, we get an initial Resize event with width and height of 2^32: PhysicalSize { width: 4294967295, height: 4294967295 }.
 *    This seems to be a bug of Glutin (/ macos 14).
 */
pub struct GlutinWindow {
    render_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>,
    target_fps: u64,
    event_listeners: WindowListeners,
}

impl Window for GlutinWindow {
    fn run(self: Box<Self>, mut renderer: Renderer, mut event_system: EventSystem, mut lz_input: Input, mut asset_manager: Box<dyn AssetManager>, mut interface: Interface) {
        let mut next_frame_time: u128 = 0;

        // Move all properties of self in to their own variables because self will get moved by event_loop.run, and thus the properties
        // can not be directly used in the function of event_loop.run
        let render_context = self.render_context;
        let mut event_listeners = self.event_listeners;
        let target_fps = self.target_fps;
        let event_loop = self.event_loop;

        // Read events from Scene::new
        Self::read_event_listeners(&mut event_listeners, &render_context.window());

        // Due to issue 1, we do not receive a correct initial size event. To fix it, we manually send one.
        event_system.send(WindowResizeEvent {
            width: interface.size().x as u32,
            height: interface.size().y as u32,
        });

        event_loop.run(move |event, _, control_flow| {
            let start_time = Instant::now();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        // Due to issue 1, we do a sanity check on the size
                        if physical_size.width > 100_000_000 || physical_size.height > 100_000_000 {
                            log::engine_warn(format!("preventing unusual window resize: {:?}", physical_size));
                            return;
                        }

                        render_context.resize(physical_size);

                        let dpi_factor = render_context.window().scale_factor();
                        let logical_size: LogicalSize<u32> = physical_size.to_logical(dpi_factor);

                        let outer_size = render_context.window().outer_size();
                        let inner_size = render_context.window().inner_size();
                        let frame_width = outer_size.width - inner_size.width;
                        let frame_height = outer_size.height - inner_size.height;

                        event_system.send(WindowResizeEvent {
                            width: logical_size.width - frame_width,
                            height: logical_size.height - frame_height,
                        });
                    },
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                        let logical_size: LogicalSize<u32> = new_inner_size.to_logical(scale_factor);

                        event_system.send(WindowResizeEvent {
                            width: logical_size.width, 
                            height: logical_size.height 
                        });
                        event_system.send(PixelDensityChangeEvent{ pixel_density: scale_factor as f32 });
                    },
                    WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                        if let Some(key) = input.virtual_keycode {
                            lz_input.keyboard.register_key_event(
                                event_mapper::map_glutin_keycode(key), 
                                event_mapper::map_glutin_key_state(input.state)
                            );
                        }
                    },
                    WindowEvent::MouseInput { device_id: _, state, button, .. } => {
                        lz_input.mouse.register_button_event(
                            event_mapper::map_glutin_mouse_button(button), 
                            event_mapper::map_glutin_mouse_button_state(state)
                        );
                    },
                    WindowEvent::MouseWheel { device_id: _, delta, phase: _, .. } => {
                        match delta {
                            glutin::event::MouseScrollDelta::LineDelta(x, y) => {
                                // TODO how is this triggered? Can we register it the same as the other type?
                                log::engine_warn(format!("TODO Untested MouseWheel event delta type: {:?}", delta));
                                lz_input.mouse.register_scroll_x_event(x as f64);
                                lz_input.mouse.register_scroll_y_event(y as f64);
                            },
                            glutin::event::MouseScrollDelta::PixelDelta(movement) => {
                                lz_input.mouse.register_scroll_x_event(movement.x as f64);
                                lz_input.mouse.register_scroll_y_event(movement.y as f64);
                            },
                        }
                    },
                    WindowEvent::CursorMoved { device_id: _, position, .. } => {
                        let logical_position: LogicalPosition<f64> = position.to_logical(render_context.window().scale_factor());
                        lz_input.mouse.register_reposition_event(logical_position.x, logical_position.y);
                    },
                    _ => (),
                },
                Event::DeviceEvent { device_id: _, event } => match event {
                    glutin::event::DeviceEvent::MouseMotion { delta } => {
                        lz_input.mouse.register_move_event(delta.0, delta.1);
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

                    interface.update(&mut *asset_manager, &lz_input);
                    renderer.scene.update(&mut event_system, &lz_input, &mut *asset_manager, &mut interface);
                    Self::read_event_listeners(&mut event_listeners, &render_context.window());

                    renderer.draw(&mut *asset_manager, &mut interface);
                    render_context.swap_buffers().expect("Failed to swap buffers");

                    lz_input.reset();

                    next_frame_time = get_next_frame_time(start_time, target_fps);
                    *control_flow = ControlFlow::Poll;
                }
            }
        });
    }

    fn get_size(&self) -> Vec2 {
        let logical_size: LogicalSize<f32> = self.render_context.window().inner_size().to_logical(self.get_pixel_density());

        return Vec2 { 
            x: logical_size.width, 
            y: logical_size.height,
        }
    }

    fn frame_size(&self) -> Vec2 {
        let outer_size: LogicalSize<f32> = self.render_context.window().outer_size().to_logical(self.get_pixel_density());
        let inner_size: LogicalSize<f32> = self.render_context.window().outer_size().to_logical(self.get_pixel_density());

        return Vec2 { 
            x: outer_size.width - inner_size.width,
            y: outer_size.height - inner_size.height,
        }
    }

    fn get_pixel_density(&self) -> f64 {
        self.render_context.window().scale_factor()
    }
}

impl GlutinWindow {
    pub fn new(window_builder: &crate::graphics::window::WindowBuilder, event_system: &mut EventSystem) -> Self {
        let mut glutin_window_builder = WindowBuilder::new()
            .with_title(window_builder.name.clone())
            .with_resizable(window_builder.resizable)
        ;

        // set window size
        glutin_window_builder = match window_builder.size {
            crate::graphics::window::WindowSize::FullScreen => {
                glutin_window_builder.with_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)))
            },
            crate::graphics::window::WindowSize::Maximized => {
                glutin_window_builder.with_maximized(true)
            },
            crate::graphics::window::WindowSize::Pixels(width, height) => {
                glutin_window_builder.with_inner_size(glutin::dpi::LogicalSize::new(width, height))
            },
        };

        let event_loop = EventLoop::new();

        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .build_windowed(glutin_window_builder, &event_loop)
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
                log::engine_warn(format!("could not lock cursor: {}",  err.to_string()));
            },
        }
    }

    fn unlock_cursor(window: &glutin::window::Window) {
        match window.set_cursor_grab(glutin::window::CursorGrabMode::None) {
            Ok(_) => (),
            Err(err) => {
                log::engine_warn(format!("could not unlock cursor: {}",  err.to_string()));
            },
        }
    }

    /// Confine the cursor the the window area
    fn confine_cursor(window: &glutin::window::Window) {
        match window.set_cursor_grab(glutin::window::CursorGrabMode::Confined) {
            Ok(_) => (),
            Err(err) => {
                log::engine_warn(format!("could not confine cursor: {}",  err.to_string()));
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
                log::engine_warn(format!("could not set cursor position: {}",  err.to_string()));
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
    let frame_time_ms = 1_000 / target_fps as u128;
    return time::now_millis() + frame_time_ms - elapsed_time;
}
