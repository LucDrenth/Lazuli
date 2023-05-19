use crate::{
    graphics::scene::Scene, 
    error::opengl
};

pub struct Renderer {
    scene: Box<dyn Scene>,
}

impl Renderer {
    pub fn new(scene: Box<dyn Scene>) -> Result<Self, String> {
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
            opengl::gl_check_errors();

            Ok(Renderer{scene})
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.45, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.scene.draw();

            opengl::gl_check_errors();
        }
    }
}
