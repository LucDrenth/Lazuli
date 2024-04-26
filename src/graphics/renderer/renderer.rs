use crate::{
    graphics::{scene::Scene, ui::Interface}, 
    error::opengl, asset_manager::AssetManager
};

use super::fps::Fps;

pub struct Renderer {
    pub scene: Box<dyn Scene>,
    fps: Fps,
}

impl Renderer {
    pub fn new(scene: Box<dyn Scene>) -> Result<Self, String> {
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);
            opengl::gl_check_errors();
        }

        Ok(Self{
            scene, fps: Fps::new(),
        })
    }

    pub fn draw(&mut self, asset_manager: &mut dyn AssetManager, interface: &Interface) {
        unsafe {
            gl::ClearColor(0.45, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.scene.draw(asset_manager);
            interface.draw(asset_manager);
            self.fps.update_fps_count();

            opengl::gl_check_errors();
        }
    }

    pub fn set_wireframe_mode(enable: bool) {
        if enable {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        } else {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
        
        opengl::gl_check_errors();
    }
}
