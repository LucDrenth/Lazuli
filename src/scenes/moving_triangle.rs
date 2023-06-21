use glam::Vec2;

use crate::{graphics::{scene::Scene, material::Material, Triangle, mesh_renderer, shader::{ShaderProgram, PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG}}, event::EventSystem, input::Input};

pub struct MovingTriangle {
    material: Material,
    triangle: Triangle,
    triangle_offset_x: f32,
    triangle_movement_velocity: f32,
}

impl Scene for MovingTriangle {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2) -> Result<Self, String> {
        let program = ShaderProgram::new(PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG).unwrap();
        let material = Material::new(program);

        let triangle = Triangle::new(&material.shader_program);

        let result = Self { 
            material,
            triangle,
            triangle_offset_x: 0.0,
            triangle_movement_velocity: 0.008,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input) {
        self.triangle_offset_x += self.triangle_movement_velocity;

        if self.triangle_offset_x > 0.5 {
            self.triangle_offset_x = 0.5;
            self.triangle_movement_velocity *= -1.0;
        }
        else if self.triangle_offset_x < -0.5 {
            self.triangle_offset_x = -0.5;
            self.triangle_movement_velocity *= -1.0;
        }

        self.material.shader_program.set_uniform("xPos", self.triangle_offset_x);
    }

    unsafe fn draw(&self) {
        mesh_renderer::draw_shape(&self.triangle, &self.material);
    }
}
