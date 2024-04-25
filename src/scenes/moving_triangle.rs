use glam::Vec2;

use crate::{asset_manager::{AssetManager, AssetManagerTrait}, event::EventSystem, graphics::{material::Material, scene::Scene, shader::{ShaderBuilder, PATH_MOVING_TRIANGLE_FRAG, PATH_MOVING_TRIANGLE_VERT}, ui::Interface, Shape, Triangle}, input::Input, ResourceId};

pub struct MovingTriangle {
    material_id: ResourceId<Material>,
    triangle: Triangle,
    triangle_offset_x: f32,
    triangle_movement_velocity: f32,
}

impl Scene for MovingTriangle {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, _pixel_density: f32, asset_manager: &mut AssetManager, _: &mut Interface) -> Result<Self, String> {
        let shader_id = asset_manager.load_shader(
            ShaderBuilder::new(PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG)
        )?;
        let material_id = asset_manager.load_material(&shader_id).unwrap();


        let triangle = Triangle::new(asset_manager.get_shader_by_id(&shader_id).unwrap());

        Ok(Self { 
            material_id,
            triangle,
            triangle_offset_x: 0.0,
            triangle_movement_velocity: 0.008,
        })
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, asset_manager: &mut AssetManager, _: &mut Interface) {
        self.triangle_offset_x += self.triangle_movement_velocity;

        if self.triangle_offset_x > 0.5 {
            self.triangle_offset_x = 0.5;
            self.triangle_movement_velocity *= -1.0;
        }
        else if self.triangle_offset_x < -0.5 {
            self.triangle_offset_x = -0.5;
            self.triangle_movement_velocity *= -1.0;
        }

        let shader_id = asset_manager.get_material_by_id(&self.material_id).unwrap().shader_id.duplicate();
        asset_manager.get_shader_by_id(&shader_id).unwrap().set_uniform("xPos", self.triangle_offset_x);
    }

    unsafe fn draw(&self, asset_manager: &mut AssetManager) {
        let shader_id = asset_manager.get_material_by_id(&self.material_id).unwrap().shader_id.duplicate();
        self.triangle.draw(asset_manager.get_shader_by_id(&shader_id).unwrap());
    }
}
