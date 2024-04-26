pub enum ImageType {
    RgbaImage (image::RgbaImage),
    GrayImage (image::GrayImage),
    Mock(),
}

impl ImageType {
    /// save the bitmap image to a file
    pub fn save(&self, path: &String) -> Result<(), String> {
        match self {
            ImageType::RgbaImage(img) => {
                img.save(path).map_err(|err| {
                    format!("Failed to save RgbaImage instance: {}", err)
                })
            },
            ImageType::GrayImage(img) => {
                img.save(path).map_err(|err| {
                    format!("Failed to save GrayImage instance: {}", err)
                })
            },
            ImageType::Mock() => Ok(()),
        }
    }
}
