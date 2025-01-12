use image::{DynamicImage, GenericImageView, ImageReader};

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

pub struct CharsetProperties {
  pub order: MatrixCharsetOrder,
  pub char_width: u32,
  pub char_height: u32,
  pub offset_left: u32,
  pub spacing_horizontal: u32,
  pub offset_top: u32,
  pub spacing_vertical: u32
}

pub enum MatrixCharsetOrder {
  /// top to bottom first
  RowInLowNibble,
  /// left to right first
  ColumnInLowNibble, 
}

pub fn load_matrix_charset(
    source: &str,
    props: &CharsetProperties,
) -> Vec<DynamicImage> {
    let charset = ImageReader::open(source)
        .expect("Unable to read image")
        .decode()
        .expect("Unable to decode image");
    let mut characters = Vec::with_capacity(0xff);
    for code in 0..0xff {
        let hn = code >> 4;
        let ln = code & 0x0f;
        let (row, column) = match props.order {
            MatrixCharsetOrder::RowInLowNibble => (ln, hn),
            MatrixCharsetOrder::ColumnInLowNibble => (hn, ln),
        };
        let x = props.offset_left + column as u32 * (props.char_width + props.spacing_horizontal);
        let y = props.offset_top + row as u32 * (props.char_height + props.spacing_vertical);
    
        characters.push(charset.crop_imm(x, y, props.char_width, props.char_height));
    }
    characters
}
