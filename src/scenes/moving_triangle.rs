use glam::Vec2;

use crate::{graphics::{scene::Scene, Triangle, shader::{PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG, ShaderBuilder}, Shape, material::Material}, event::EventSystem, input::Input, asset_registry::{AssetRegistry, AssetId}};

pub struct MovingTriangle {
    material_id: AssetId<Material>,
    triangle: Triangle,
    triangle_offset_x: f32,
    triangle_movement_velocity: f32,
}

impl Scene for MovingTriangle {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> {
        let shader_id = asset_registry.load_shader(ShaderBuilder::new()
            .with_vertex_shader_path(PATH_MOVING_TRIANGLE_VERT.to_string())
            .with_fragment_shader_path(PATH_MOVING_TRIANGLE_FRAG.to_string())
        ).unwrap();
        let material_id = asset_registry.load_material(&shader_id).unwrap();


        let triangle = Triangle::new(asset_registry.get_shader_by_id(&shader_id).unwrap());

        Ok(Self { 
            material_id,
            triangle,
            triangle_offset_x: 0.0,
            triangle_movement_velocity: 0.008,
        })
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, asset_registry: &mut AssetRegistry) {
        self.triangle_offset_x += self.triangle_movement_velocity;

        if self.triangle_offset_x > 0.5 {
            self.triangle_offset_x = 0.5;
            self.triangle_movement_velocity *= -1.0;
        }
        else if self.triangle_offset_x < -0.5 {
            self.triangle_offset_x = -0.5;
            self.triangle_movement_velocity *= -1.0;
        }

        let shader_id = asset_registry.get_material_by_id(&self.material_id).unwrap().shader_id.duplicate();
        asset_registry.get_shader_by_id(&shader_id).unwrap().set_uniform("xPos", self.triangle_offset_x);
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        let shader_id = asset_registry.get_material_by_id(&self.material_id).unwrap().shader_id.duplicate();
        self.triangle.draw(asset_registry.get_shader_by_id(&shader_id).unwrap());
    }
}
