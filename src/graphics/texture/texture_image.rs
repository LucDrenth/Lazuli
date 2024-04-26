use std::os::raw::c_uint;

use gl::types::GLenum;
use image::{EncodableLayout, GrayImage, RgbaImage};

use super::ImageType;

pub trait TextureImage {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn bytes(&self) -> *const u8;
    fn format(&self) -> c_uint;
}

#[derive(Debug)]
pub struct GlTextureImage {
    pub width: u32,
    pub height: u32,
    pub bytes: *const u8,
    pub format: GLenum,
}

impl TextureImage for GlTextureImage {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn format(&self) -> c_uint {
        return self.format
    }

    fn bytes(&self) -> *const u8 {
        self.bytes
    }
}

impl From<&RgbaImage> for GlTextureImage {
    fn from(img: &RgbaImage) -> Self {
        Self {
            width: img.width(),
            height: img.height(),
            bytes: img.as_bytes().as_ptr(),
            format: gl::RGBA,
        }
    }
}
impl From<&GrayImage> for GlTextureImage {
    fn from(img: &GrayImage) -> Self {
        Self {
            width: img.width(),
            height: img.height(),
            bytes: img.as_bytes().as_ptr(),
            format: gl::RED,
        }
    }
}
impl From<&ImageType> for GlTextureImage {
    fn from(img: &ImageType) -> Self {
        match img {
            ImageType::RgbaImage(img) => {
                return Self::from(img);
            },
            ImageType::GrayImage(img) => {
                return Self::from(img);
            },
            ImageType::Mock() => panic!("Can not convert ImageType Mock to GlTextureImage"),
        }
    }
}
