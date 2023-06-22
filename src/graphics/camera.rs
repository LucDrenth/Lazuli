use glam::{Mat4, Vec3};

// TODO move to its own module. Move projection and view in to a seperate file.

/// Rotation limits in degrees, all >= 0
pub struct LookDirectionLimits {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct Projection {
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
    fov: Fov,
}

pub struct ZoomLimits {
    max_zoom_in: f32, 
    max_zoom_out: f32,
}

struct Fov {
    base: f32,
    zoom: f32,
    zoom_limits: ZoomLimits,
}

pub struct Camera {
    projection: Projection,
    pub position: Vec3,
    pub pitch: f32, // horizontal rotation in degrees
    pub yaw: f32, // vertical rotation in degrees
    pub look_sensetivity: f32,
    direction: Vec3, // the center of the camera
    look_direction_limits: Option<LookDirectionLimits>,
    invert_y_axis: f32, // 1 for true, -1 for false
    up_direction: Vec3,
}

impl Camera {   
    pub fn new(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Self {
        let fov = Fov {
            base: fov,
            zoom: 0.0,
            zoom_limits: ZoomLimits { 
                max_zoom_in: 5.0,
                max_zoom_out: fov * 1.5,
            }
        };

        let projection = Projection {
            aspect_ratio,
            near_plane,
            far_plane,
            fov,
        };

        let mut camera = Camera {
            projection,
            position: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
            look_sensetivity: 1.0,
            direction: Vec3::ZERO,
            look_direction_limits: None,
            invert_y_axis: -1.0,
            up_direction: Vec3{x: 0.0, y: 1.0, z: 0.0},
        };
        camera.set_direction();

        camera
    }

    pub fn projection_for_shader(&self) -> Mat4 {
        return self.projection.for_shader();
    }

    pub fn view_for_shader(&self) -> Mat4 {
        return Mat4::look_at_rh(self.position, self.position + self.direction, self.up_direction);
    }

    pub fn look_at(&mut self, direction: Vec3) {
        // TODO set pitch and yaw, then call set_direction(). Otherwise, rotating resets the camera direction to the last used rotation.
        self.direction = Vec3::from(direction - self.position);
    }

    pub fn set_rotation_limits(&mut self, limits: LookDirectionLimits) {
        self.look_direction_limits = Some(limits);
    }

    pub fn remove_rotation_limits(&mut self) {
        self.look_direction_limits = None;
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

    fn check_look_direction_limits(&mut self) {
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

    fn set_direction(&mut self) {
        self.check_look_direction_limits();

        let yaw = (self.yaw - 90.0).to_radians(); // subtract 90 to look on front by default
        let pitch = self.invert_y_axis * self.pitch.to_radians();

        self.direction = Vec3 {
            x: yaw.cos() * pitch.cos(),
            y: pitch.sin(),
            z: yaw.sin() * pitch.cos(),
        };
    }


    pub fn move_forth(&mut self, amount: f32) {
        self.position += amount * self.direction;
    }

    pub fn move_back(&mut self, amount : f32) {
        self.move_forth(-amount);
    }

    pub fn move_horizontal(&mut self, amount: f32) {
        self.position += self.direction.cross(self.up_direction) * amount;
    }

    pub fn move_left(&mut self, amount: f32) {
        self.move_horizontal(-amount);
    }

    pub fn move_right(&mut self, amount: f32) {
        self.move_horizontal(amount);
    }

    pub fn move_up(&mut self, amount: f32) {
        self.position.y += amount;
    }

    pub fn move_down(&mut self, amount: f32) {
        self.position.y -= amount;
    }

    pub fn zoom(&mut self, amount: f32) {
        self.projection.fov.zoom -= amount;
        self.projection.fov.confine_zoom_limits();
    }
}

impl Fov {
    fn confine_zoom_limits(&mut self) {
        if self.base + self.zoom > self.zoom_limits.max_zoom_out {
            self.zoom = self.zoom_limits.max_zoom_out - self.base;
        } else if self.base + self.zoom < self.zoom_limits.max_zoom_in {
            self.zoom = self.zoom_limits.max_zoom_in - self.base;
        }
    }
}

impl Projection {
    pub fn for_shader(&self) -> Mat4 {
        Mat4::perspective_rh_gl(
            (self.fov.base + self.fov.zoom).to_radians(), 
            self.aspect_ratio, 
            self.near_plane, 
            self.far_plane
        )
    }
}
