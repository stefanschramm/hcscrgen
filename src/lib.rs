use image::DynamicImage;
use profiles::{kc87::Kc87Converter, sharpmz::SharpMzConverter};

mod profiles;
mod utils;

pub enum Profile {
    SharpMz,
    Kc87,
}

trait Converter {
    fn new() -> Self;
    fn convert(&self, input_img: &DynamicImage) -> Result<ConversionResult, String>;
}

pub struct ConversionResult {
    pub preview: DynamicImage,
    pub character_ram: Vec<u8>,
    pub color_ram: Option<Vec<u8>>,
}

pub fn convert(input_img: &DynamicImage, profile: Profile) -> Result<ConversionResult, String> {
    match profile {
        Profile::SharpMz => SharpMzConverter::new().convert(input_img),
        Profile::Kc87 => Kc87Converter::new().convert(input_img)
    }
}
