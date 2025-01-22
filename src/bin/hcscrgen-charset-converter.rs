use clap::{arg, Parser};
use image::{GenericImage, ImageReader, Rgb, RgbImage};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Character width
    #[arg(long, default_value_t = 8)]
    width: u32,
    /// Character height
    #[arg(long, default_value_t = 8)]
    height: u32,
    /// Offset top
    #[arg(long, default_value_t = 0)]
    top: u32,
    /// Offset left
    #[arg(long, default_value_t = 0)]
    left: u32,
    /// Vertical spacing
    #[arg(long, default_value_t = 1)]
    vertical_spacing: u32,
    /// Horizontal spacing
    #[arg(long, default_value_t = 1)]
    horizontal_spacing: u32,
    /// Arrangement of characters (lr: left to right, tb: top to bottom)
    #[arg(long, default_value = "lr")]
    mode: String,
    /// Input file
    #[arg()]
    input_file: String,
    /// Output file
    #[arg()]
    output_file: String,
}

enum Mode {
    LeftRight,
    TopBottom,
}

fn main() {
    let args = Args::parse();

    let charset = import_charset(&args);
    store_charset(&args, &charset);

    println!("len: {}", charset.len());
}

fn import_charset(args: &Args) -> Vec<RgbImage> {
    let mode = match args.mode.as_str() {
        "tb" => Mode::TopBottom,
        "lr" => Mode::LeftRight,
        _ => panic!("Invalid mode. Expected tb or tr."),
    };

    let charset = ImageReader::open(&args.input_file)
        .expect("Unable to open input file")
        .decode()
        .expect("Unable to decode image");

    let mut characters = Vec::with_capacity(0x100);

    for code in 0..0x100 {
        let hn = code >> 4;
        let ln = code & 0x0f;
        let (row, column) = match mode {
            Mode::TopBottom => (ln, hn),
            Mode::LeftRight => (hn, ln),
        };
        let x = args.left + column as u32 * (args.width + args.horizontal_spacing);
        let y = args.top + row as u32 * (args.height + args.vertical_spacing);

        characters.push(charset.crop_imm(x, y, args.width, args.height).into_rgb8());
    }

    characters
}

fn store_charset(args: &Args, charset: &Vec<RgbImage>) {
    let mut output = RgbImage::from_pixel(
        1 + (args.width + 1) * 16,
        1 + (args.height + 1) * 16,
        Rgb([0x80, 0x80, 0x80]),
    );

    for (index, character) in charset.iter().enumerate() {
        let row = index as u32 / 16;
        let column = index as u32 % 16;
        let x = 1 + column * (args.width + 1);
        let y = 1 + row * (args.height + 1);
        output
            .copy_from(character, x, y)
            .expect("Unable to copy character into destination image");
    }

    output
        .save(&args.output_file)
        .expect("Unable to store destination image");
}
