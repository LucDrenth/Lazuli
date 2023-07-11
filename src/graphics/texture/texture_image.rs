use gl::types::GLenum;
use image::{RgbaImage, GrayImage, EncodableLayout};

use super::ImageType;

pub struct TextureImage {
    pub width: u32,
    pub height: u32,
    pub bytes: *const u8,
    pub format: GLenum,
}

// TODO abstractiate this
impl From<&RgbaImage> for TextureImage {
    fn from(img: &RgbaImage) -> Self {
        TextureImage {
            width: img.width(),
            height: img.height(),
            bytes: img.as_bytes().as_ptr(),
            format: gl::RGBA,
        }
    }
}
impl From<&GrayImage> for TextureImage {
    fn from(img: &GrayImage) -> Self {
        TextureImage {
            width: img.width(),
            height: img.height(),
            bytes: img.as_bytes().as_ptr(),
            format: gl::RED,
        }
    }
}
impl From<&ImageType> for TextureImage {
    fn from(img: &ImageType) -> Self {
        match img {
            ImageType::RgbaImage(img) => {
                return Self::from(img);
            },
            ImageType::GrayImage(img) => {
                return Self::from(img);
            },
        }
    }
}
