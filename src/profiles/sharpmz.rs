use image::{DynamicImage, GenericImage};

use crate::{
    utils::{image_diff, load_matrix_charset, CharsetProperties},
    ConversionResult, Converter,
};

const CHAR_HEIGHT: u32 = 8;
const CHAR_WIDTH: u32 = 8;
const COLUMNS: u32 = 40; // characters
const LINES: u32 = 25; // characters
const SCREEN_WIDTH: u32 = COLUMNS * CHAR_WIDTH;
const SCREEN_HEIGHT: u32 = LINES * CHAR_HEIGHT;

/**
 * https://original.sharpmz.org/mz-700/colorvram.htm
 * https://original.sharpmz.org/mz-700/codetable.htm
 */
pub struct SharpMzConverter {
    charset: Vec<DynamicImage>,
    charset_extended: Vec<DynamicImage>,
}

// TODO: Put some of of the methods into more generic places

impl Converter for SharpMzConverter {
    fn new() -> Self {
        let props = CharsetProperties {
            order: crate::utils::MatrixCharsetOrder::RowInLowNibble,
            char_width: CHAR_WIDTH,
            char_height: CHAR_HEIGHT,
            offset_left: 2,
            spacing_horizontal: 3,
            offset_top: 2,
            spacing_vertical: 3,
        };

        Self {
            charset: load_matrix_charset("profiles/sharp_mz-700/charset.png", &props),
            charset_extended: load_matrix_charset(
                "profiles/sharp_mz-700/charset_extended.png",
                &props,
            ),
        }
    }

    fn convert(&self, input_img: &DynamicImage) -> Result<crate::ConversionResult, String> {
        if input_img.width() < SCREEN_WIDTH || input_img.height() < SCREEN_HEIGHT {
            return Err(format!(
                "Input file must have a dimension of at least {}x{} pixels.",
                SCREEN_WIDTH, SCREEN_HEIGHT
            ));
        }

        let mut codes: Vec<u16> = Vec::new();

        for row in 0..LINES {
            for column in 0..COLUMNS {
                let tile = input_img.crop_imm(
                    column * CHAR_WIDTH,
                    row * CHAR_HEIGHT,
                    CHAR_WIDTH,
                    CHAR_HEIGHT,
                );
                codes.push(self.get_matching_code(&tile));
            }
        }

        Ok(ConversionResult {
            preview: self.create_preview(&codes),
            character_ram: SharpMzConverter::get_character_ram(&codes),
            color_ram: Some(SharpMzConverter::get_color_ram(&codes)),
        })
    }
}

impl SharpMzConverter {
    fn get_matching_code(&self, tile: &DynamicImage) -> u16 {
        let mut best_code: u16 = 0;
        let mut best_diff: u32 = u32::MAX;
        // TODO: match colors
        for code in 0_u8..0xff_u8 {
            let diff = image_diff(&tile, &self.charset[code as usize]);
            if diff < best_diff {
                best_code = code as u16;
                best_diff = diff;
            }
        }
        for code in 0_u8..0xff_u8 {
            let diff = image_diff(&tile, &self.charset_extended[code as usize]);
            if diff < best_diff {
                best_code = 0x8000 | code as u16;
                best_diff = diff;
            }
        }
        best_code | 0x0700 // fg color
    }

    fn create_preview(&self, codes: &Vec<u16>) -> DynamicImage {
        let mut preview_img = DynamicImage::new_rgb8(SCREEN_WIDTH, SCREEN_HEIGHT);
        for row in 0..LINES {
            for column in 0..COLUMNS {
                let code: u16 = codes[(row * COLUMNS + column) as usize];
                let charset = if code & 0x8000 != 0 {
                    &self.charset_extended
                } else {
                    &self.charset
                };
                let charset_code = (code & 0xff) as u8;

                preview_img
                    .copy_from(
                        &charset[charset_code as usize],
                        column * CHAR_WIDTH,
                        row * CHAR_HEIGHT,
                    )
                    .expect("Unable to put tile into preview image");
            }
        }

        preview_img
    }

    pub fn get_character_ram(codes: &Vec<u16>) -> Vec<u8> {
        codes.iter().map(|&c| (c & 0x00ff) as u8).collect()
    }

    pub fn get_color_ram(codes: &Vec<u16>) -> Vec<u8> {
        codes.iter().map(|&c| (c >> 8) as u8).collect()
    }
}
