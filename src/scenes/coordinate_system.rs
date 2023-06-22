use std::f32::consts::{PI, TAU};

use glam::{Vec3, Vec2};
use rand::{Rng, rngs::ThreadRng};

use crate::{graphics::{scene::Scene, material::Material, Cube, mesh_renderer, shader::{ShaderProgram, PATH_COLORED_FRAG}, Transform, Camera, LookDirectionLimits}, event::EventSystem, input::{Input, Key}, time, lz_core_info};

pub struct CoordinateSystem {
    material: Material,
    cubes: Vec<Cube>,
    transforms: Vec<Transform>,
    rotations: Vec<Vec3>,
    camera: Camera,
    rng: ThreadRng,
    movement_speed: f32,
}

impl Scene for CoordinateSystem {
    fn new(_event_system: &mut EventSystem, window_size: Vec2) -> Result<Self, String> {
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
            transform.translate_z(rng.gen_range(10.0..30.0));
            transform.translate_x(rng.gen_range(0.0..10.0) - 5.0);
            transform.translate_y(rng.gen_range(0.0..10.0) - 5.0);
            transform.rotate_x(rng.gen_range(0.0..TAU) - PI);
            transform.rotate_x(rng.gen_range(0.0..TAU) - PI);
            transforms.push(transform);

            rotations.push(Vec3 { 
                x: (rng.gen_range(0.0..10.0) - 5.0) / 250.0, 
                y: (rng.gen_range(0.0..10.0) - 5.0) / 375.0, 
                z: 0.0,
            });
        }

        let mut camera = Camera::new(window_size.x / window_size.y, 45.0, 0.1, 100.0);
        // camera.set_rotation_limits(LookDirectionLimits {left: 20.0, right: 20.0, top: 20.0, bottom: 20.0});
        camera.look_sensetivity = 3.0;
        camera.position.z -= -40.0;
        material.shader_program.set_uniform("projection", camera.projection_for_shader());
        material.shader_program.set_uniform("view", camera.view_for_shader());

        let result = Self { 
            material,
            cubes,
            transforms,
            rotations,
            camera,
            rng,
            movement_speed: 10.0,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input) {
        for i in 0..self.cubes.len() {
            self.transforms[i].rotate(&self.rotations[i]);
        }

        // self.poll_axis_movement(input);
        self.poll_free_movement(input);
        
        if input.did_mouse_move() {
            self.camera.rotate(input.get_mouse_moved_x() as f32 / 50.0, input.get_mouse_moved_y() as f32 / 50.0);
        }

        if input.is_key_down(Key::Space) {
            self.camera.look_at(self.transforms[self.rng.gen_range(0..self.cubes.len())].position);
        }

        self.material.shader_program.set_uniform("view", self.camera.view_for_shader());
    }

    unsafe fn draw(&self) {
        for i in 0..self.cubes.len() {
            self.material.shader_program.set_uniform("model", self.transforms[i].for_shader());
            mesh_renderer::draw_shape(&self.cubes[i], &self.material);
        }
    }
}

impl CoordinateSystem {
    fn poll_free_movement(&mut self, input: &Input) {
        if input.is_key_held(Key::A) {
            self.camera.move_left(self.movement_speed * time::DELTA);
        }
        if input.is_key_held(Key::D) {
            self.camera.move_right(self.movement_speed * time::DELTA);
        }
        if input.is_key_held(Key::S) {
            self.camera.move_back(self.movement_speed * time::DELTA);
        }
        if input.is_key_held(Key::W) {
            self.camera.move_forth(self.movement_speed * time::DELTA);
        }
        if input.is_key_held(Key::Shift) {
            self.camera.move_up(self.movement_speed * time::DELTA);
        }
        if input.is_key_held(Key::Cntrl) {
            self.camera.move_down(self.movement_speed * time::DELTA);
        }
    }
    
    fn poll_axis_movement(&mut self, input: &Input) {
        let mut direction_x: f32 = 0.0;
        let mut direction_y: f32 = 0.0;
        let mut direction_z: f32 = 0.0;

        if input.is_key_held(Key::A) {
            direction_x -= 1.0;
        }
        if input.is_key_held(Key::D) {
            direction_x += 1.0;
        }
        if input.is_key_held(Key::S) {
            direction_z += 1.0;
        }
        if input.is_key_held(Key::W) {
            direction_z -= 1.0;
        }
        if input.is_key_held(Key::Shift) {
            direction_y += 1.0;
        }
        if input.is_key_held(Key::Cntrl) {
            direction_y -= 1.0;
        }

        self.camera.position += Vec3::new(
            direction_x * self.movement_speed * time::DELTA,
            direction_y * self.movement_speed * time::DELTA,
            direction_z * self.movement_speed * time::DELTA,
        );
    }
}
