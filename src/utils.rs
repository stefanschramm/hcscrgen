use std::io::Cursor;

use image::{DynamicImage, GenericImageView, ImageReader};

use crate::profiles::{CharsetDefinition, MatrixCharsetOrder};

pub fn image_diff(a: &DynamicImage, b: &DynamicImage) -> u32 {
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

pub fn load_matrix_charset(
    charset_data: &[u8],
    def: &CharsetDefinition,
) -> Vec<DynamicImage> {
    let cursor = Cursor::new(charset_data);
    let reader = ImageReader::with_format(cursor, image::ImageFormat::Png);
    let charset = reader
        .decode()
        .expect("Unable to decode charset image");
    let mut characters = Vec::with_capacity(0xff);
    for code in 0..0xff {
        let hn = code >> 4;
        let ln = code & 0x0f;
        let (row, column) = match def.mode {
            MatrixCharsetOrder::RowInLowNibble => (ln, hn),
            MatrixCharsetOrder::ColumnInLowNibble => (hn, ln),
        };
        let x = def.offset_left + column as u32 * (def.character_width + def.spacing_horizontal);
        let y = def.offset_top + row as u32 * (def.character_height + def.spacing_vertical);
    
        characters.push(charset.crop_imm(x, y, def.character_width, def.character_height));
    }
    characters
}
