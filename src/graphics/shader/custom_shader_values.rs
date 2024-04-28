use std::collections::HashMap;

use glam::{Vec4, Vec3, Vec2};

use super::{uniform::UniformValue, ShaderProgram};

#[derive(Clone)]
pub struct CustomShaderValues {
    uniforms_vec2: HashMap<String, Vec2>,
    uniforms_vec3: HashMap<String, Vec3>,
    uniforms_vec4: HashMap<String, Vec4>,
    uniforms_f32: HashMap<String, f32>,
}

impl CustomShaderValues {
    pub fn set_vec2(&mut self, name: impl Into<String>, value: Vec2) {
        self.uniforms_vec2.insert(name.into(), value);
    }
    pub fn set_vec3(&mut self, name: impl Into<String>, value: Vec3) {
        self.uniforms_vec3.insert(name.into(), value);
    }
    pub fn set_vec4(&mut self, name: impl Into<String>, value: Vec4) {
        self.uniforms_vec4.insert(name.into(), value);
    }

    pub fn set_f32(&mut self, name: impl Into<String>, value: f32) {
        self.uniforms_f32.insert(name.into(), value);
    }

    pub fn upload(&self, shader: &Box<dyn ShaderProgram>) {
        for (name, value) in self.uniforms_vec2.iter() {
            shader.set_uniform(name, &UniformValue::from(*value));
        }
        for (name, value) in self.uniforms_vec3.iter() {
            shader.set_uniform(name, &UniformValue::from(*value));
        }
        for (name, value) in self.uniforms_vec4.iter() {
            shader.set_uniform(name, &UniformValue::from(*value));
        }

        for (name, value) in self.uniforms_f32.iter() {
            shader.set_uniform(name, &UniformValue::from(*value));
        }
    }
}

impl Default for CustomShaderValues {
    fn default() -> Self {
        Self { 
            uniforms_vec2: Default::default(),
            uniforms_vec3: Default::default(),
            uniforms_vec4: Default::default(),
            uniforms_f32: Default::default(),
        }
    }
}
