use super::TextureImage;

pub struct MockTextureImage {
    pub width: u32,
    pub height: u32,
}

impl TextureImage for MockTextureImage {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn bytes(&self) -> *const u8 {
        std::ptr::null()
    }

    fn format(&self) -> std::os::raw::c_uint {
        0
    }
}
