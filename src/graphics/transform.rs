use glam::{Vec3, Mat4};

pub struct Transform {
    pub position: Vec3, // AKA view.
    pub rotation: Vec3, // AKA model. A Vec3 of radians. TODO use a quaternion instead.
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

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
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

    pub fn translate(&mut self, amount: &Vec3) {
        self.position.x += amount.x;
        self.position.y += amount.y;
        self.position.z += amount.z;
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

    /// rotate by Vec3 of radians
    pub fn rotate(&mut self, rotation: &Vec3) {
        self.rotation.x += rotation.x;
        self.rotation.y += rotation.y;
        self.rotation.z += rotation.z;
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

    pub fn scale(&mut self, scale: &Vec3) {
        self.scale.x += scale.x;
        self.scale.y += scale.y;
        self.scale.z += scale.z;
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

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale.x = scale.x;
        self.scale.y = scale.y;
        self.scale.z = scale.z;
    }

    pub fn build(&self) -> Mat4 {
        return self.get_scale_matrix() * self.get_view_matrix() * self.get_model_matrix();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.position)
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        return Mat4::from_rotation_x(self.rotation.x) 
            * Mat4::from_rotation_y(self.rotation.y) 
            * Mat4::from_rotation_y(self.rotation.y);
    }

    pub fn get_scale_matrix(&self) -> Mat4 {
        return Mat4::from_scale(self.scale);
    }
}
