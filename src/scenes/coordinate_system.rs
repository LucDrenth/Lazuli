use crate::{graphics::{scene::Scene, material::Material, Triangle, mesh_renderer, shader::{ShaderProgram, PATH_MOVING_TRIANGLE_FRAG}, Transform, Camera}, event::EventSystem, lz_core_info};

use glam::Vec3;

pub struct CoordinateSystem {
    material: Material,
    triangle: Triangle,
    triangle_model_matrix: Transform,
}

impl Scene for CoordinateSystem {
    fn new(_event_system: &mut EventSystem) -> Result<Self, String> {
        let program = ShaderProgram::new("./assets/shaders/with-camera.vert", PATH_MOVING_TRIANGLE_FRAG).unwrap();
        let material = Material::new(program);
        let triangle = Triangle::new(&material.shader_program);

        let camera = Camera::new(1000.0 / 750.0, 45.0, 0.1, 100.0);

        let rot_x: f32 = -55.0;
        let triangle_model_matrix = Transform { 
            position: Vec3 { x: 0.0, y: 0.0, z: -3.0 }, 
            rotation: Vec3 { x: rot_x.to_radians(), y: 0.0, z: 0.0 }, 
            scale: Vec3::ONE,
        };

        material.shader_program.set_uniform("camera", camera.for_shader());
        material.shader_program.set_uniform("transform", triangle_model_matrix.build());

        let result = Self { 
            material,
            triangle,
            triangle_model_matrix,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem) {}

    unsafe fn draw(&self) {
        mesh_renderer::draw_triangle(&self.triangle, &self.material);
    }
}
