use crate::graphics::{shapes::{Triangle, Rectangle, Shape}, material::Material};

pub fn draw_rectangle(mesh: &Rectangle, material: &Material) {
    material.activate();
    unsafe {
        mesh.draw(&material.shader_program);
    }
}

pub fn draw_triangle(mesh: &Triangle, material: &Material) {
    material.activate();
    unsafe {
        mesh.draw(&material.shader_program);
    }
}
