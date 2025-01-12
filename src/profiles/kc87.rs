use image::{DynamicImage, GenericImage};

use crate::{
    utils::{image_diff, load_matrix_charset, CharsetProperties},
    ConversionResult, Converter,
};

const CHAR_HEIGHT: u32 = 8;
const CHAR_WIDTH: u32 = 8;
const COLUMNS: u32 = 40; // characters
const LINES: u32 = 24; // characters
const SCREEN_WIDTH: u32 = COLUMNS * CHAR_WIDTH;
const SCREEN_HEIGHT: u32 = LINES * CHAR_HEIGHT;

pub struct Kc87Converter {
    charset: Vec<DynamicImage>,
}

/**
 * https://hc-ddr.hucki.net/wiki/doku.php/z9001/versionen
 */
impl Converter for Kc87Converter {
    fn new() -> Self {
        let props = CharsetProperties {
            order: crate::utils::MatrixCharsetOrder::ColumnInLowNibble,
            char_width: CHAR_WIDTH,
            char_height: CHAR_HEIGHT,
            offset_left: 1,
            spacing_horizontal: 1,
            offset_top: 1,
            spacing_vertical: 1,
        };

        Self {
            charset: load_matrix_charset("profiles/kc87/charset.gif", &props),
        }
    }

    fn convert(&self, input_img: &image::DynamicImage) -> Result<crate::ConversionResult, String> {
        // TODO: put check in common function
        if input_img.width() < SCREEN_WIDTH || input_img.height() < SCREEN_HEIGHT {
            return Err(format!(
                "Input file must have a dimension of at least {}x{} pixels.",
                SCREEN_WIDTH, SCREEN_HEIGHT
            ));
        }

        let mut codes: Vec<u8> = Vec::new();

        // TODO: iterator into common function
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
            character_ram: codes,
            color_ram: None,
        })
    }
}

impl Kc87Converter {
    fn get_matching_code(&self, tile: &DynamicImage) -> u8 {
        let mut best_code: u8 = 0;
        let mut best_diff: u32 = u32::MAX;
        // TODO: match colors
        for code in 0_u8..0xff_u8 {
            let diff = image_diff(&tile, &self.charset[code as usize]);
            if diff < best_diff {
                best_code = code;
                best_diff = diff;
            }
        }
        best_code
    }

    fn create_preview(&self, codes: &Vec<u8>) -> DynamicImage {
        let mut preview_img = DynamicImage::new_rgb8(SCREEN_WIDTH, SCREEN_HEIGHT);
        for row in 0..LINES {
            for column in 0..COLUMNS {
                let code: u8 = codes[(row * COLUMNS + column) as usize];
                preview_img
                    .copy_from(
                        &self.charset[code as usize],
                        column * CHAR_WIDTH,
                        row * CHAR_HEIGHT,
                    )
                    .expect("Unable to put tile into preview image");
            }
        }

        preview_img
    }
}
