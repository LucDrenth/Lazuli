use glam::{Vec3, Mat4};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // in radians. TODO use a quaternion
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform { position: Vec3::ZERO, rotation: Vec3::ZERO, scale: Vec3::ONE }
    }

    pub fn set_position_x(&mut self, position: f32) {
        self.position.x = position;
    }

    pub fn set_position_y(&mut self, position: f32) {
        self.position.y = position;
    }

    pub fn set_position_z(&mut self, position: f32) {
        self.position.z = position;
    }

    pub fn translate_x(&mut self, amount: f32) {
        self.position.x += amount;
    }

    pub fn translate_y(&mut self, amount: f32) {
        self.position.y += amount;
    }

    pub fn translate_z(&mut self, amount: f32) {
        self.position.z += amount;
    }

    /// rotate in radians
    pub fn rotate_x(&mut self, rotation: f32) {
        self.rotation.x += rotation;
    }

    /// rotate in radians
    pub fn rotate_y(&mut self, rotation: f32) {
        self.rotation.y += rotation;
    }

    /// rotate in radians
    pub fn rotate_z(&mut self, rotation: f32) {
        self.rotation.z += rotation;
    }

    pub fn scale(&mut self, scale: f32) {
        self.scale.x += scale;
        self.scale.y += scale;
        self.scale.z += scale;
    }

    pub fn scale_x(&mut self, scale: f32) {
        self.scale.x += scale;
    }

    pub fn scale_y(&mut self, scale: f32) {
        self.scale.y += scale;
    }

    pub fn scale_z(&mut self, scale: f32) {
        self.scale.z += scale;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale.x = scale;
        self.scale.y = scale;
        self.scale.z = scale;
    }

    pub fn set_scale_x(&mut self, scale: f32) {
        self.scale.x = scale;
    }

    pub fn set_scale_y(&mut self, scale: f32) {
        self.scale.y = scale;
    }

    pub fn set_scale_z(&mut self, scale: f32) {
        self.scale.z = scale;
    }

    pub fn build(&self) -> Mat4 {
        let mut result = Mat4::IDENTITY;

        // TODO use Quaternions for the rotations
        result = Mat4::from_rotation_x(self.rotation.x) * result;
        result = Mat4::from_rotation_y(self.rotation.y) * result;
        result = Mat4::from_rotation_z(self.rotation.z) * result;

        result = Mat4::from_translation(self.position) * result;

        result = Mat4::from_scale(self.scale) * result;

        return result;
    }
}
