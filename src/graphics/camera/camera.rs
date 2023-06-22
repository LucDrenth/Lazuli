use glam::{Mat4, Vec3};

use super::{projection::Projection, view::View, LookDirectionLimits};

pub struct Camera {
    projection: Projection,
    view: View,
}

impl Camera {   
    pub fn new(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Self {
        Camera {
            projection: Projection::new(aspect_ratio, fov, near_plane, far_plane),
            view: View::new(),
        }
    }

    pub fn projection_for_shader(&self) -> Mat4 {
        return self.projection.for_shader();
    }
    pub fn view_for_shader(&self) -> Mat4 {
        return self.view.for_shader()
    }

    pub fn look_at(&mut self, direction: Vec3) {
        self.view.look_at(direction);
    }
    pub fn set_rotation_limits(&mut self, limits: LookDirectionLimits) {
        self.view.look_direction_limits = Some(limits);
    }
    pub fn remove_rotation_limits(&mut self) {
        self.view.look_direction_limits = None;
    }
    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.view.rotate(yaw, pitch);
    }
    pub fn pitch(&mut self, pitch: f32) {
        self.view.pitch(pitch);
    }
    pub fn yaw(&mut self, yaw: f32) {
        self.view.yaw(yaw);
    }

    pub fn move_forth(&mut self, amount: f32) {
        self.view.move_forth(amount);
    }
    pub fn move_back(&mut self, amount : f32) {
        self.move_forth(-amount);
    }
    pub fn move_horizontal(&mut self, amount: f32) {
        self.view.move_horizontal(amount);
    }
    pub fn move_left(&mut self, amount: f32) {
        self.move_horizontal(-amount);
    }
    pub fn move_right(&mut self, amount: f32) {
        self.move_horizontal(amount);
    }
    pub fn move_up(&mut self, amount: f32) {
        self.view.position.y += amount;
    }
    pub fn move_down(&mut self, amount: f32) {
        self.view.position.y -= amount;
    }

    pub fn zoom(&mut self, amount: f32) {
        self.projection.zoom(amount);
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.view.position = position;
    }
    pub fn set_position_x(&mut self, x: f32) {
        self.view.position.x = x;
    }
    pub fn set_position_y(&mut self, y: f32) {
        self.view.position.y = y;
    }
    pub fn set_position_z(&mut self, z: f32) {
        self.view.position.z = z;
    }
    pub fn translate(&mut self, amount: Vec3) {
        self.view.position += amount;
    }
    pub fn translate_x(&mut self, amount: f32) {
        self.view.position.x += amount;
    }
    pub fn translate_y(&mut self, amount: f32) {
        self.view.position.y += amount;
    }
    pub fn translate_z(&mut self, amount: f32) {
        self.view.position.z += amount;
    }

    pub fn set_look_sensitivity(&mut self, amount: f32) {
        self.view.look_sensetivity = amount;
    }
    pub fn get_look_sensitivity(&self) -> f32 {
        self.view.look_sensetivity
    }
}
