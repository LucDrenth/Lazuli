use glam::Vec2;

use crate::{graphics::{scene::Scene, material::Material, Cube, mesh_renderer, shader::{ShaderProgram, PATH_COLORED_FRAG}, Transform, Camera, text::Font}, event::{EventSystem, self}, input::{Input, Key}, time};

pub struct HelloText {
    material: Material,
    cube: Cube,
    cube_transform: Transform,
    camera: Camera,
    movement_speed: f32,
    zoom_speed: f32,
}

impl Scene for HelloText {
    fn new(event_system: &mut EventSystem, window_size: Vec2) -> Result<Self, String> {
        /////////////////////
        // START TEXT TEST //
        ////////////////////

        let f = Font::new("./assets/fonts/roboto.ttf".to_string(), 25.0)?;

        /////////////////////
        //  END TEXT TEST  //
        /////////////////////

        event_system.send(event::HideCursor{});
        event_system.send(event::LockCursor{});

        let program = ShaderProgram::new("./assets/shaders/with-camera.vert", PATH_COLORED_FRAG).unwrap();
        let material = Material::new(program);

        let cube = Cube::new_colored(&material.shader_program);
        let mut cube_transform = Transform::new();
        cube_transform.rotate_x(2.2);
        cube_transform.rotate_y(2.0);

        let mut camera = Camera::new(window_size.x / window_size.y, 45.0, 0.1, 500.0);
        camera.set_look_sensitivity(3.0);
        camera.translate_z(-40.0);
        material.shader_program.set_uniform("projection", camera.projection_for_shader());
        material.shader_program.set_uniform("view", camera.view_for_shader());

        let result = Self { 
            material,
            cube,
            cube_transform,
            camera,
            movement_speed: 10.0,
            zoom_speed: 10.0,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input) {
        self.poll_free_movement(input);
        self.poll_zoom(input);
        if input.did_mouse_move() {
            self.camera.rotate(input.get_mouse_moved_x() as f32 / 50.0, input.get_mouse_moved_y() as f32 / 50.0);
        }

        if input.is_key_down(Key::Space) {
            self.camera.look_at(self.cube_transform.position);
        }

        self.material.shader_program.set_uniform("view", self.camera.view_for_shader());
    }

    unsafe fn draw(&self) {
        self.material.shader_program.set_uniform("model", self.cube_transform.for_shader());
        mesh_renderer::draw_shape(&self.cube, &self.material);
    }
}

impl HelloText {
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

    fn poll_zoom(&mut self, input: &Input) {
        let scroll_y = input.get_scroll_y() as f32 * self.zoom_speed * time::DELTA;

        if scroll_y != 0.0 {
            self.camera.zoom(scroll_y);
            self.material.shader_program.set_uniform("projection", self.camera.projection_for_shader());
        }
    }
}
