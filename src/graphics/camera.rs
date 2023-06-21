use glam::{Mat4, Vec3};

pub struct Camera {
    projection_matrix: Mat4,
    pub position: Vec3,
    pub look_at: Vec3,
}

impl Camera {   
    pub fn new(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Self {
        let projection_matrix = Mat4::perspective_rh_gl(fov.to_radians(), aspect_ratio, near_plane, far_plane);

        Camera {
            projection_matrix,
            position: Vec3::ZERO,
            look_at: Vec3::ZERO,
        }
    }

    pub fn projection_for_shader(&self) -> Mat4 {
        return self.projection_matrix;
    }

    pub fn view_for_shader(&self) -> Mat4 {
        return Mat4::look_at_rh(self.position, self.look_at, Vec3{x: 0.0, y: 1.0, z: 0.0});
    }
}
