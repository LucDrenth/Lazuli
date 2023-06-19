use crate::graphics::{shapes::{Shape}, material::Material};

pub fn draw_shape(mesh: &dyn Shape, material: &Material) {
    material.activate();
    unsafe {
        mesh.draw(&material.shader_program);
    }
}