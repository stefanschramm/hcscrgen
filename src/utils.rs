use std::io::Cursor;

use image::{ImageReader, RgbImage};

pub struct CharsetDefinition {
    pub character_width: u32,
    pub character_height: u32,
}

pub fn image_diff(a: &RgbImage, b: &RgbImage) -> u32 {
    assert!(a.width() == b.width() && a.height() == b.height());
    let width = a.width();
    let height = a.height();
    let mut diff: u32 = 0;
    for x in 0..width {
        for y in 0..height {
            let pa = a.get_pixel(x, y);
            let pb = b.get_pixel(x, y);
            diff = diff
                + pa[0].abs_diff(pb[0]) as u32
                + pa[1].abs_diff(pb[1]) as u32
                + pa[2].abs_diff(pb[2]) as u32;
        }
    }

    diff / (width * height)
}

pub fn load_charset(charset_data: &[u8], def: &CharsetDefinition) -> Vec<RgbImage> {
    let cursor = Cursor::new(charset_data);
    let reader = ImageReader::with_format(cursor, image::ImageFormat::Png);
    let charset = reader.decode().expect("Unable to decode charset image");
    let mut characters = Vec::with_capacity(0x100);
    for code in 0..0x100 {
        let row = code >> 4;
        let column = code & 0x0f;
        let x = 1 + column as u32 * (def.character_width + 1);
        let y = 1 + row as u32 * (def.character_height + 1);

        characters.push(
            charset
                .crop_imm(x, y, def.character_width, def.character_height)
                .into_rgb8(),
        );
    }
    characters
}
