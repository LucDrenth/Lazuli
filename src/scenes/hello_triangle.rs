use crate::{graphics::{scene::Scene, material::Material, Triangle, mesh_renderer, shader::{ShaderProgram, PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG}}, event::EventSystem};

pub struct HelloTriangle {
    material: Material,
    triangle: Triangle,
}

impl Scene for HelloTriangle {
    fn new(_event_system: &mut EventSystem) -> Result<Self, String> {
        let program = ShaderProgram::new(PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG).unwrap();
        let material = Material::new(program);

        let triangle = Triangle::new(&material.shader_program);

        let result = Self { 
            material,
            triangle,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem) {}

    unsafe fn draw(&self) {
        mesh_renderer::draw_shape(&self.triangle, &self.material);
    }
}
