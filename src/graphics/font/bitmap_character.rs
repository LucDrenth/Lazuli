use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct BitmapCharacter {
    pub texture_start_x: f32,
    pub texture_end_x: f32,
    pub texture_start_y: f32,
    pub texture_end_y: f32,
    pub width: f32, // relative to the lineheight of the font
}
