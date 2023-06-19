use crate::{graphics::{scene::Scene, material::Material, Triangle, Cube, mesh_renderer, shader::{ShaderProgram, PATH_MOVING_TRIANGLE_FRAG}, Transform, Camera}, event::EventSystem, lz_core_info};

use glam::Vec3;

pub struct CoordinateSystem {
    material: Material,
    model_matrix: Transform,
    cube: Cube,
}

impl Scene for CoordinateSystem {
    fn new(_event_system: &mut EventSystem) -> Result<Self, String> {
        let program = ShaderProgram::new("./assets/shaders/with-camera.vert", PATH_MOVING_TRIANGLE_FRAG).unwrap();
        let material = Material::new(program);

        let cube = Cube::new_colored(&material.shader_program);

        let camera = Camera::new(1000.0 / 750.0, 45.0, 0.1, 100.0);

        let rot_x: f32 = -55.0;
        let model_matrix = Transform { 
            position: Vec3 { x: 0.0, y: 0.0, z: -3.0 }, 
            rotation: Vec3 { x: rot_x.to_radians(), y: 0.0, z: 0.0 }, 
            scale: Vec3::ONE,
        };

        material.shader_program.set_uniform("camera", camera.for_shader());
        material.shader_program.set_uniform("transform", model_matrix.build());

        let result = Self { 
            material,
            model_matrix,
            cube,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem) {
        self.model_matrix.rotate_x(0.005);
        self.model_matrix.rotate_y(0.01);
        self.material.shader_program.set_uniform("transform", self.model_matrix.build());
    }

    unsafe fn draw(&self) {
        mesh_renderer::draw_shape(&self.cube, &self.material);
    }
}
