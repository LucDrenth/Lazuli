use glam::Mat4;

pub struct Projection {
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub fov: Fov,
}

pub struct ZoomLimits {
    pub max_zoom_in: f32, 
    pub max_zoom_out: f32,
}

pub struct Fov {
    pub base: f32,
    pub zoom: f32,
    pub zoom_limits: ZoomLimits,
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
    pub fn new(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Self {
        Projection {
            aspect_ratio,
            near_plane,
            far_plane,
            fov: Fov {
                base: fov,
                zoom: 0.0,
                zoom_limits: ZoomLimits { 
                    max_zoom_in: 5.0,
                    max_zoom_out: fov * 1.5,
                }
            },
        }
    }

    pub fn for_shader(&self) -> Mat4 {
        Mat4::perspective_rh_gl(
            (self.fov.base + self.fov.zoom).to_radians(), 
            self.aspect_ratio, 
            self.near_plane, 
            self.far_plane
        )
    }

    pub fn zoom(&mut self, amount: f32) {
        self.fov.zoom -= amount;
        self.fov.confine_zoom_limits();
    }
}
