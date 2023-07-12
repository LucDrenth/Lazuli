use image::{GrayImage, Luma, RgbaImage, Rgba};

use crate::lz_core_warn;

/// Create a new, downsampled image by averaging the values of each block (factor x factor) of the input image
/// 
/// # Arguments
/// 
/// * `image` - 
/// * `factor` - reduce both with and height by this amount, e.g. factor 4 is 1024x1024 to 256x256
/// * `method` - 
pub fn downsample_gray_image(image: &GrayImage, factor: u32) -> GrayImage {
    if factor < 1 {
        lz_core_warn!("invalid image downsample factor of {}. Returning input image clone", factor);
        return image.clone();
    }

    if image.width() % factor != 0 || image.height() % factor != 0 {
        lz_core_warn!(
            "downsampling image of {}x{} with factor {} is not perfectly dividible, which may lead to ugly borders",
            image.width(), image.height(), factor,
        );
    }

    let mut result: GrayImage = GrayImage::new(image.width() / factor, image.height() / factor as u32);

    for x in 0..result.width() {
        for y in 0..result.height() {
            let average = gray_image_average(image, factor, x * factor, y * factor);
            result.put_pixel(x, y, Luma([average]));
        }
    }


    result
}

fn gray_image_average(image: &GrayImage, factor: u32, start_x: u32, start_y: u32) -> u8 {
    let mut total: u16 = 0;

    for x in start_x..start_x+factor {
        for y in start_y..start_y+factor {
            total += image.get_pixel(x, y).0[0] as u16;
        }
    }

    let average = total as f32 / (factor * factor) as f32;

    return average.min(255.0) as u8
}

/// Create a new, downsampled image by averaging the color values of each block (factor x factor) of the input image
/// 
/// # Arguments
/// 
/// * `image` - 
/// * `factor` - reduce both with and height by this amount, e.g. factor 4 is 1024x1024 to 256x256
/// * `method` - 
pub fn downsample_rgba_image(image: &RgbaImage, factor: u32) -> RgbaImage {
    if factor < 1 {
        lz_core_warn!("invalid image downsample factor of {}. Returning input image clone", factor);
        return image.clone();
    }

    if image.width() % factor != 0 || image.height() % factor != 0 {
        lz_core_warn!(
            "downsampling image of {}x{} with factor {} is not perfectly dividible, which may lead to ugly borders",
            image.width(), image.height(), factor,
        );
    }

    let mut result: RgbaImage = RgbaImage::new(image.width() / factor, image.height() / factor as u32);

    for x in 0..result.width() {
        for y in 0..result.height() {
            let average = rgba_image_average(image, factor, x * factor, y * factor);
            result.put_pixel(x, y, Rgba(average));
        }
    }


    result
}

fn rgba_image_average(image: &RgbaImage, factor: u32, start_x: u32, start_y: u32) -> [u8; 4] {
    let mut total_r: u16 = 0;
    let mut total_g: u16 = 0;
    let mut total_b: u16 = 0;
    let mut total_a: u16 = 0;
    
    for x in start_x..start_x+factor {
        for y in start_y..start_y+factor {
            total_r += image.get_pixel(x, y).0[0] as u16;
            total_g += image.get_pixel(x, y).0[1] as u16;
            total_b += image.get_pixel(x, y).0[2] as u16;
            total_a += image.get_pixel(x, y).0[3] as u16;
        }
    }

    let average_r = total_r as f32 / (factor * factor) as f32;
    let average_g = total_g as f32 / (factor * factor) as f32;
    let average_b = total_b as f32 / (factor * factor) as f32;
    let average_a = total_a as f32 / (factor * factor) as f32;

    return [
        average_r.min(255.0) as u8,
        average_g.min(255.0) as u8,
        average_b.min(255.0) as u8,
        average_a.min(255.0) as u8,
    ]
}
