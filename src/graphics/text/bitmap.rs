use image::{DynamicImage, Rgba, RgbaImage, ImageBuffer};
use rusttype::PositionedGlyph;

pub struct Bitmap {
    image: RgbaImage,
    characters: Vec<BitmapCharacter>,
}

impl Bitmap {
    pub fn new(font: &rusttype::Font<'static>, font_size: f32, padding: u32) -> Result<Bitmap, String> {
        let (image, characters) = create(font, font_size, padding).map_err(|err| {
            format!("Failed to create bitmap: {}", err)
        })?;
        Ok(Self { image, characters })
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

pub struct BitmapCharacter {
    character: char,
    start_x: i32,
    start_y: i32,
    width: i32,
}

pub fn create(font: &rusttype::Font<'static>, font_size: f32, padding: u32) -> Result<(RgbaImage, Vec<BitmapCharacter>), String> {
    let text = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!№;%:?*()_+-=.,/|\\\"'@#$€^&{}[]";
    let colour = (255, 255, 255); // white

    let scale = rusttype::Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);
    let start_point = rusttype::point(padding as f32, padding as f32 + v_metrics.ascent);
    let glyphs: Vec<_> = font.layout(text, scale, start_point).collect();

    let glyphs_width = glyphs.last().take().unwrap().pixel_bounding_box().unwrap().max.x as u32;
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

    let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = DynamicImage::new_rgba8(
        glyphs_width + padding * 2, 
        glyphs_height + padding * 2
    ).to_rgba8();
    let mut bitmap_characters: Vec<BitmapCharacter> = Vec::with_capacity(text.len());
    write_glyphs(glyphs, text, &mut image_buffer, &mut bitmap_characters, colour, padding);

    let rgba_image = to_rgba_image(image_buffer)?;
    Ok((rgba_image, bitmap_characters))
}

fn write_glyphs(
    glyphs: Vec<PositionedGlyph<'_>>, 
    characters: &str, 
    image_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, 
    bitmap_characters: &mut Vec<BitmapCharacter>,
    colour: (u8, u8, u8),
    padding: u32,
) {
    for (i, glyph) in glyphs.iter().enumerate() {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            bitmap_characters.push(BitmapCharacter {
                character: characters.chars().nth(i).take().unwrap(),
                start_x: bounding_box.min.x + padding as i32,
                start_y: padding as i32,
                width: bounding_box.max.x - bounding_box.min.x,
            });

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

fn to_rgba_image(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<RgbaImage, String> {
    let (width, height) = image_buffer.dimensions();
    let raw_pixels = image_buffer.into_raw();
    
    match RgbaImage::from_raw(width, height, raw_pixels) {
        Some(img) => Ok(img),
        None => Err(format!("Could not create RgbaImage")),
    }
}
