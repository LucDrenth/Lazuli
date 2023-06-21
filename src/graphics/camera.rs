use glam::{Mat4, Vec3};

pub struct Camera {
    projection_matrix: Mat4,
    pub position: Vec3,
}

impl Camera {   
    pub fn new(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Self {
        let projection_matrix = Mat4::perspective_rh_gl(fov.to_radians(), aspect_ratio, near_plane, far_plane);
        
        Camera {
            projection_matrix: projection_matrix,
            position: Vec3::ZERO,
        }
    }

    pub fn for_shader(&self) -> Mat4 {
        return self.projection_matrix * Mat4::from_translation(self.position);
    }
}
