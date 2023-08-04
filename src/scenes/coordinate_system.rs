use std::f32::consts::{PI, TAU};

use glam::{Vec3, Vec2};
use rand::{Rng, rngs::ThreadRng};

use crate::{graphics::{scene::Scene, Cube, shader::{PATH_COLORED_FRAG, ShaderBuilder}, Transform, Camera, Shape, material::Material}, event::{EventSystem, self}, input::{Input, Key}, time, asset_manager::{AssetManager, AssetId}};

pub struct CoordinateSystem {
    material_id: AssetId<Material>,
    cubes: Vec<Cube>,
    transforms: Vec<Transform>,
    rotations: Vec<Vec3>,
    camera: Camera,
    rng: ThreadRng,
    movement_speed: f32,
    zoom_speed: f32,
}

impl Scene for CoordinateSystem {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_manager: &mut AssetManager) -> Result<Self, String> {
        event_system.send(event::LockCursor{});
        event_system.send(event::HideCursor{});

        let shader_id = asset_manager.load_shader(ShaderBuilder::new()
            .with_vertex_shader_path("./assets/shaders/with-camera.vert".to_string())
            .with_fragment_shader_path(PATH_COLORED_FRAG.to_string())
        ).unwrap();
        let material_id = asset_manager.load_material(&shader_id).unwrap();

        let mut cubes = vec![];
        let mut transforms = vec![];
        let mut rotations = vec![];

        let mut rng = rand::thread_rng();

        for _ in 0..15 {
            let cube = Cube::new_colored(asset_manager.get_shader_by_id(&shader_id).unwrap());
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

        let mut camera = Camera::new(window_size.x / window_size.y, 45.0, 0.1, 500.0);
        camera.set_look_sensitivity(3.0);
        camera.translate_z(-40.0);

        {
            let shader = asset_manager.get_shader_by_id(&shader_id).unwrap();
            shader.set_uniform("projection", camera.projection_for_shader());
            shader.set_uniform("view", camera.view_for_shader());
        }

        let result = Self { 
            material_id,
            cubes,
            transforms,
            rotations,
            camera,
            rng,
            movement_speed: 10.0,
            zoom_speed: 10.0,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_manager: &mut AssetManager) {
        for i in 0..self.cubes.len() {
            self.transforms[i].rotate(&self.rotations[i]);
        }

        // self.poll_axis_movement(input);
        self.poll_free_movement(input);
        self.poll_zoom(input, asset_manager);
        
        if input.did_mouse_move() {
            self.camera.rotate(input.get_mouse_moved_x() as f32 / 50.0, input.get_mouse_moved_y() as f32 / 50.0);
        }

        if input.is_key_down(Key::Space) {
            self.camera.look_at(self.transforms[0].position);
        }

        asset_manager.get_material_shader(&self.material_id).unwrap().set_uniform("view", self.camera.view_for_shader());
    }

    unsafe fn draw(&self, asset_manager: &mut AssetManager) {
        let shader = asset_manager.get_material_shader(&self.material_id).unwrap();

        for i in 0..self.cubes.len() {
            shader.set_uniform("model", self.transforms[i].for_shader());
            self.cubes[i].draw(shader);
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

        if input.is_key_held(Key::T) {
            self.camera.move_towards(self.transforms[0].position, self.movement_speed * time::DELTA);
        }
        if input.is_key_held(Key::G) {
            self.camera.move_away_from(self.transforms[0].position, self.movement_speed * time::DELTA);
        }
    }

    fn poll_zoom(&mut self, input: &Input, asset_manager: &mut AssetManager) {
        let scroll_y = input.get_scroll_y() as f32 * self.zoom_speed * time::DELTA;

        if scroll_y != 0.0 {
            self.camera.zoom(scroll_y);
            asset_manager.get_material_shader(&self.material_id).unwrap().set_uniform("projection", self.camera.projection_for_shader());
        }
    }
}
