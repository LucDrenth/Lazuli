use glam::{Mat4};

pub struct Camera {
    projection_matrix: Mat4
}

impl Camera {   
    pub fn new(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Self {
        Camera {
            projection_matrix: Mat4::perspective_rh_gl(fov.to_radians(), aspect_ratio, near_plane, far_plane)
        }
    }

    pub fn for_shader(&self) -> Mat4 {
        return self.projection_matrix;
    }
}
