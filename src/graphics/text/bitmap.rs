use image::{DynamicImage, Rgba, RgbaImage};

pub struct Bitmap {
    image: RgbaImage,
}

impl Bitmap {
    pub fn new(font: &rusttype::Font<'static>, font_size: f32, padding: u32) -> Result<Bitmap, String> {
        let image = create(font, font_size, padding).map_err(|err| {
            format!("Failed to create bitmap: {}", err)
        })?;
        Ok(Self { image })
    }

    /// save the bitmap image to a file
    pub fn save(&self, path: &String) -> Result<(), String> {
        let save_result = self.image.save(path).map_err(|err| {
            format!("Failed to save bitmap image: {}", err)
        });

        match save_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// TODO - padding is not 100% accurate
/// TODO - automatic size / wrapping
pub fn create(font: &rusttype::Font<'static>, font_size: f32, padding: u32) -> Result<RgbaImage, String> {
    let scale = rusttype::Scale::uniform(font_size);
    let text = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!№;%:?*()_+-=.,/|\\\"'@#$^&{}[]";

    let colour = (255, 255, 255); // white

    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(text, scale, rusttype::point(padding as f32, padding as f32 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    let mut image_buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>> = DynamicImage::new_rgba8(glyphs_width + padding * 2, glyphs_height + padding * 2).to_rgba8();
    write_glyphs(glyphs, &mut image_buffer, colour);

    to_rgba_image(image_buffer)
}

fn write_glyphs(glyphs: Vec<rusttype::PositionedGlyph<'_>>, image_buffer: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, colour: (u8, u8, u8)) {
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image_buffer.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                )
            });
        }
    }
}

fn to_rgba_image(image_buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<RgbaImage, String> {
    let (width, height) = image_buffer.dimensions();
    let raw_pixels = image_buffer.into_raw();
    
    match RgbaImage::from_raw(width, height, raw_pixels) {
        Some(img) => Ok(img),
        None => Err(format!("Could not create RgbaImage")),
    }
}
