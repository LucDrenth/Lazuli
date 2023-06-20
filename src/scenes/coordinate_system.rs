use std::f32::consts::{PI, TAU};

use glam::Vec3;
use rand::Rng;

use crate::{graphics::{scene::Scene, material::Material, Cube, mesh_renderer, shader::{ShaderProgram, PATH_COLORED_FRAG}, Transform, Camera}, event::EventSystem, input::{Input, Key}, lz_core_info};

pub struct CoordinateSystem {
    material: Material,
    cubes: Vec<Cube>,
    transforms: Vec<Transform>,
    rotations: Vec<Vec3>,
}

impl Scene for CoordinateSystem {
    fn new(_event_system: &mut EventSystem) -> Result<Self, String> {
        let program = ShaderProgram::new("./assets/shaders/with-camera.vert", PATH_COLORED_FRAG).unwrap();
        let material = Material::new(program);

        let mut cubes = vec![];
        let mut transforms = vec![];
        let mut rotations = vec![];

        let mut rng = rand::thread_rng();

        for _ in 0..15 {
            let cube = Cube::new_colored(&material.shader_program);
            cubes.push(cube);

            let mut transform = Transform::new();
            transform.translate_z(rng.gen_range(3.0..30.0) - 33.0);
            transform.translate_x(rng.gen_range(0.0..10.0) - 5.0);
            transform.translate_y(rng.gen_range(0.0..10.0) - 5.0);
            transform.rotate_x(rng.gen_range(0.0..TAU) - PI);
            transform.rotate_x(rng.gen_range(0.0..TAU) - PI);
            transforms.push(transform);

            rotations.push(Vec3 { 
                x: (rng.gen_range(0.0..10.0) - 5.0) / 200.0, 
                y: (rng.gen_range(0.0..10.0) - 5.0) / 300.0, 
                z: 0.0,
            });
        }

        let camera = Camera::new(1000.0 / 750.0, 45.0, 0.1, 100.0);
        material.shader_program.set_uniform("camera", camera.for_shader());

        let result = Self { 
            material,
            cubes,
            transforms,
            rotations,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _input: &Input) {
        for i in 0..self.cubes.len() {
            self.transforms[i].rotate(&self.rotations[i]);
        }
    }

    unsafe fn draw(&self) {
        for i in 0..self.cubes.len() {
            self.material.shader_program.set_uniform("transform", self.transforms[i].build());
            mesh_renderer::draw_shape(&self.cubes[i], &self.material);
        }
    }
}
