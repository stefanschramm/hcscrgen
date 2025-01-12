use std::{env, fs::File, io::Write, process};

use hcscrgen::convert;
use image::{EncodableLayout, ImageReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} INPUTFILE", args[0]);
        process::exit(1);
    }
    let input_file = &args[1];

    let input_img = ImageReader::open(input_file)
        .expect("Unable to read image")
        .decode()
        .expect("Unable to decode image");

    // TODO: Does it work with KC 87?
    // match convert(&input_img, hcscrgen::Profile::Kc87) {
    match convert(&input_img, hcscrgen::Profile::SharpMz) {
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
        }
    }
}
