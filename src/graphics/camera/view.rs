use glam::{Vec3, Mat4};

pub struct View {
    pub position: Vec3,
    pub pitch: f32, // horizontal rotation in degrees
    pub yaw: f32, // vertical rotation in degrees
    pub look_sensetivity: f32,
    pub direction: Vec3, // the center of the camera
    pub look_direction_limits: Option<LookDirectionLimits>,
    pub invert_y_axis: f32, // 1 for true, -1 for false
    pub up_direction: Vec3,
}

/// Rotation limits in degrees, all >= 0
pub struct LookDirectionLimits {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl View {
    pub fn new() -> Self {
        let mut view = View {
            position: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
            look_sensetivity: 1.0,
            direction: Vec3::ZERO,
            look_direction_limits: None,
            invert_y_axis: -1.0,
            up_direction: Vec3{x: 0.0, y: 1.0, z: 0.0},
        };
        view.set_direction();

        view
    }

    pub fn for_shader(&self) -> Mat4 {
        return Mat4::look_at_rh(self.position, self.position + self.direction, self.up_direction);
    }

    pub fn set_direction(&mut self) {
        self.confine_look_direction_limits();

        let yaw = (self.yaw + 90.0).to_radians();
        let pitch = self.invert_y_axis * self.pitch.to_radians();

        self.direction = Vec3 {
            x: yaw.cos() * pitch.cos(),
            y: pitch.sin(),
            z: yaw.sin() * pitch.cos(),
        };
    }

    fn confine_look_direction_limits(&mut self) {
        if let Some(limit) = &self.look_direction_limits {
            if self.yaw < -limit.left {
                self.yaw = -limit.left;
            } else if self.yaw > limit.right {
                self.yaw = limit.right;
            }

            if self.pitch * self.invert_y_axis > limit.bottom {
                self.pitch = limit.bottom * self.invert_y_axis;
            } else if self.pitch * self.invert_y_axis < -limit.top {
                self.pitch = -limit.top * self.invert_y_axis;
            }
        }
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.pitch += pitch * self.look_sensetivity;
        self.yaw += yaw * self.look_sensetivity;
        self.set_direction();
    }

    pub fn pitch(&mut self, pitch: f32) {
        self.pitch += pitch * self.look_sensetivity;
        self.set_direction();
    }

    pub fn yaw(&mut self, yaw: f32) {
        self.yaw += yaw * self.look_sensetivity;
        self.set_direction();
    }
}
