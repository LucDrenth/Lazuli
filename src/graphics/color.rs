use crate::log;

#[derive(Debug, Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
    Hex(String),
}

impl Color {
    pub fn hex(hex: impl Into<String>) -> Self {
        return Self::Hex(hex.into())
    }

    /// Ignores any alpha value
    pub fn to_rgb_tuple(&self) -> (u8, u8, u8) {
        match self {
            Color::Rgb(r, g, b) => (*r, *g, *b),
            Color::Rgba(r, g, b, _) => (*r, *g, *b),
            Color::Hex(hex) => hex_to_rgb(hex),
        }
    }

    /// Ignores any alpha value
    pub fn to_normalised_rgb_tuple(&self) -> (f32, f32, f32) {
        let tuple = self.to_rgb_tuple();

        return (
            tuple.0 as f32 / 255.0,
            tuple.1 as f32 / 255.0,
            tuple.2 as f32 / 255.0,
        );
    }

    /// If the value does not have any alpha, alpha will be set to 1.0
    pub fn to_rgba_tuple(&self) -> (u8, u8, u8, f32) {
        match self {
            Color::Rgb(r, g, b) => (*r, *g, *b, 1.0),
            Color::Rgba(r, g, b, a) => (*r, *g, *b, *a),
            Color::Hex(hex) => hex_to_rgba(hex),
        }
    }

    /// Can be used in shaders (vec4)
    pub fn to_normalised_rgba_tuple(&self) -> (f32, f32, f32, f32) {
        let tuple = self.to_rgba_tuple();

        return (
            tuple.0 as f32 / 255.0,
            tuple.1 as f32 / 255.0,
            tuple.2 as f32 / 255.0,
            tuple.3,
        );
    }

    pub fn set_opacity(&self, opacity: f32) -> Self {
        match self {
            Color::Rgb(r, g, b) => Self::Rgba(*r, *g, *b, opacity),
            Color::Rgba(r, g, b, _) => Self::Rgba(*r, *g, *b, opacity),
            Color::Hex(hex) => {
                let rgb = hex_to_rgb(hex);
                Self::Rgba(rgb.0, rgb.1, rgb.2, opacity)

                // TODO return `Self::Hex`. For this, we'll need to have a function 
                // that converts a f32 (opacity) to a 2 character hexadecimal code
            },
        }
    }

    pub fn opacity(&self) -> f32 {
        match self {
            Color::Rgb(_, _, _) => 1.0,
            Color::Rgba(_, _, _, a) => *a,
            Color::Hex(_) => todo!(),
        }
    }

    pub fn transparent() -> Self { Self::Rgba(255, 255, 255, 0.0) }
    pub fn rgba_white() -> Self { Self::Rgba(255, 255, 255, 1.0) }
    pub fn rgb_white() -> Self { Self::Rgb(255, 255, 255) }
    pub fn hex_white() -> Self { Self::hex("#FFFFFF") }
    pub fn rgba_black() -> Self { Self::Rgba(0, 0, 0, 1.0) }
    pub fn rgb_black() -> Self { Self::Rgb(0, 0, 0) }
    pub fn hex_black() -> Self { Self::hex("#000000") }
    pub fn rgba_gray() -> Self { Self::Rgba(122, 122, 122, 1.0) }
    pub fn rgb_gray() -> Self { Self::Rgb(122, 122, 122) }
    pub fn hex_gray() -> Self { Self::hex("#888888") }
    pub fn rgb_red() -> Self { Self::Rgb(255, 0, 0) }
    pub fn rgba_red() -> Self { Self::Rgba(255, 0, 0, 1.0) }
    pub fn hex_red() -> Self { Self::hex("#FF0000") }
    pub fn rgb_yellow() -> Self { Self::Rgb(255, 255, 0) }
    pub fn rgba_yellow() -> Self { Self::Rgba(255, 255, 0, 1.0) }
    pub fn hex_yellow() -> Self { Self::hex("#FFFF00") }
}

pub fn hex_to_rgb(hex: &String) -> (u8, u8, u8) {
    match is_valid_hex(hex) {
        Ok(_) => (),
        Err(err) => {
            log::engine_err(format!("hex_to_rgb failed: {}", err));
            return (0, 0, 0)
        },
    }

    (
        u8::from_str_radix(&hex[1..3], 16).unwrap(),
        u8::from_str_radix(&hex[3..5], 16).unwrap(),
        u8::from_str_radix(&hex[5..7], 16).unwrap(),
    )
}

/// If the input does not have any alpha, alpha will be set to 1.0
pub fn hex_to_rgba(hex: &String) -> (u8, u8, u8, f32) {
    match is_valid_hex(hex) {
        Ok(_) => (),
        Err(err) => {
            log::engine_err(format!("hex_to_rgba failed: {}", err));
            return (0, 0, 0, 1.0)
        },
    }
    
    let rgb = hex_to_rgb(hex);

    let alpha;
    if hex_has_alpha(hex) {
        let alpha_u8 = u8::from_str_radix(&hex[hex.len() - 2..], 16).unwrap();
        alpha = alpha_u8 as f32 / 255.0;
    } else {
        alpha = 1.0;
    };
    
    return (rgb.0, rgb.1, rgb.2, alpha)
}

pub fn is_valid_hex(hex: &String) -> Result<(), String> {
    if !hex.starts_with('#') {
        return Err(format!("hex color {} does not start with '#'", hex));
    }

    if !hex.len() == 7 && hex.len() == 9 {
        return Err(format!("hex color {} has invalid length {}", hex, hex.len()));
    }

    Ok(())
}

pub fn hex_has_alpha(hex: &String) -> bool {
    hex.len() == 9
}
