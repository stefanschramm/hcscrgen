use std::{fs::File, io::Write};

use clap::Parser;
use hcscrgen::convert;
use image::{EncodableLayout, ImageReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Generate charset
    #[arg(short, long)]
    generate_charset: bool,
    /// Machine profile to use
    #[arg()]
    profile: String,
    /// Image file to convert
    #[arg()]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    let input_file = &args.input_file;

    let input_img = ImageReader::open(input_file)
        .expect("Unable to read image")
        .decode()
        .expect("Unable to decode image");

    match convert(&input_img, &args.profile, args.generate_charset) {
        Err(error_message) => {
            eprintln!("Error while converting: {}", error_message);
            return;
        }
        Ok(result) => {
            result
                .preview
                .save(format!("{}.preview.png", input_file))
                .expect("Unable to store preview image.");

            File::create(format!("{}.chars.bin", input_file))
                .expect("Unable to open character ram output file.")
                .write_all(&result.character_ram.as_bytes())
                .expect("Unable to write to character ram output file.");

            if let Some(color_ram) = result.color_ram {
                File::create(format!("{}.color.bin", input_file))
                    .expect("Unable to open color ram output file.")
                    .write_all(&color_ram.as_bytes())
                    .expect("Unable to write to color ram output file.");
            }

            if let Some(charset) = result.charset {
                File::create(format!("{}.charset.bin", input_file))
                    .expect("Unable to open charset output file.")
                    .write_all(&charset.as_bytes())
                    .expect("Unable to write to charset output file.");
            }
        }
    }
}
