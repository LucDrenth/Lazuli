use crate::{graphics::{scene::Scene, material::Material, Triangle, mesh_renderer, shader::{Shader, ShaderProgram, PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG}}, lz_info};

pub struct MovingTriangleScene {
    material_colored: Material,
    triangle: Triangle,
    triangle_offset_x: f32,
    triangle_movement_velocity: f32,
}

impl MovingTriangleScene {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let vertex_shader = Shader::new(PATH_MOVING_TRIANGLE_VERT, gl::VERTEX_SHADER).unwrap();
            let fragment_shader = Shader::new(PATH_MOVING_TRIANGLE_FRAG, gl::FRAGMENT_SHADER).unwrap();
            let program_colored = ShaderProgram::new(&[vertex_shader, fragment_shader]).unwrap();
            let material_colored = Material::new(program_colored);

            let triangle = Triangle::new(&material_colored.shader_program);

            let result = Self { 
                material_colored,
                triangle,
                triangle_offset_x: 0.0,
                triangle_movement_velocity: 0.008,
            };

            Ok(result)
        }
    }
}

impl Scene for MovingTriangleScene {
    fn update(&mut self) {
        self.triangle_offset_x += self.triangle_movement_velocity;

        if self.triangle_offset_x > 0.5 {
            self.triangle_offset_x = 0.5;
            self.triangle_movement_velocity *= -1.0;
        }
        else if self.triangle_offset_x < -0.5 {
            self.triangle_offset_x = -0.5;
            self.triangle_movement_velocity *= -1.0;
        }

        self.material_colored.shader_program.set_uniform_float("xPos", self.triangle_offset_x);
    }

    unsafe fn draw(&self) {
        mesh_renderer::draw_triangle(&self.triangle, &self.material_colored);
    }
}
