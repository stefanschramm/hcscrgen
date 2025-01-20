use crate::{profiles::MachineProfile, utils::image_diff};
use image::{DynamicImage, GenericImage, Pixel, Rgb, RgbImage};
use kmeans::{Cluster, KmeansContext};
use rand::random;

mod kmeans;

/// Generate an optimized charset for representing the passed image
pub fn generate_charset(profile: &ScreenProfile, img: &DynamicImage) -> Vec<RgbImage> {
    let tiles = initialize_tiles(&profile, &img);

    let context = CharsetGeneratorContext {
        character_width: profile.character_width,
        character_height: profile.character_height,
        _columns: profile.columns,
        _lines: profile.lines,
    };

    let clusters = kmeans::optimize(&context, profile.chars, &tiles, 50, 5);

    // context.draw_approximation("final.png", &clusters);

    clusters
        .iter()
        .map(|cluster| cluster.centroid.clone())
        .collect()
}

/// Converts a charset that is represented as vector of images to memory representation as expected by the C64
pub fn convert_charset(characters: &Vec<RgbImage>) -> Vec<u8> {
    assert!(characters.len() == 256);
    let mut data: Vec<u8> = Vec::with_capacity(255 * 8);
    for character in characters {
        assert!(character.width() == 8 && character.height() == 8);
        for y in 0..8 {
            let mut byte: u8 = 0;
            for x in 0..8 {
                let pixel = character.get_pixel(x, y);
                let avg = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
                if avg > 0x80 {
                    byte |= 1 << (7 - x);
                }
            }
            data.push(byte);
        }
    }
    data
}

struct CharsetGeneratorContext {
    character_width: u32,
    character_height: u32,
    _columns: u32,
    _lines: u32,
}

impl CharsetGeneratorContext {
    fn _draw_approximation(&self, filename: &str, clusters: &Vec<Cluster<RgbImage>>) -> () {
        let mut approxed_img = RgbImage::new(
            self.character_width * self._columns,
            self.character_height * self._lines,
        );
        for cluster in clusters {
            for element in &cluster.elements {
                let line = element.index as u32 / self._columns;
                let column = element.index as u32 - (line * self._columns);
                let y = line * self.character_height;
                let x = column * self.character_width;

                approxed_img
                    .copy_from(&cluster.centroid, x, y)
                    .expect("Unable to copy to approxed img");
            }
        }
        approxed_img
            .save(filename)
            .expect("Unable to write approxed img");
    }

    fn _determine_monochrome_centroid(&self, elements: &Vec<&RgbImage>) -> RgbImage {
        let mut img = RgbImage::new(self.character_width, self.character_height);
        for x in 0..self.character_width {
            for y in 0..self.character_height {
                let mut pixels_set = 0;
                for element in elements {
                    let pixel = element.get_pixel(x, y).to_rgb();
                    if (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3 > 0x80 {
                        pixels_set += 1;
                    }
                }
                if pixels_set > elements.len() / 2 {
                    img.put_pixel(x, y, Rgb([0xff, 0xff, 0xff]));
                }
            }
        }

        img
    }

    fn determine_colored_centroid(&self, elements: &Vec<&RgbImage>) -> RgbImage {
        let mut img = RgbImage::new(self.character_width, self.character_height);

        for x in 0..self.character_width {
            for y in 0..self.character_height {
                let r_avg = elements
                    .iter()
                    .map(|element| element.get_pixel(x, y).to_rgb()[0] as u32)
                    .sum::<u32>() as u32
                    / elements.len() as u32;
                let g_avg = elements
                    .iter()
                    .map(|element| element.get_pixel(x, y).to_rgb()[1] as u32)
                    .sum::<u32>() as u32
                    / elements.len() as u32;
                let b_avg = elements
                    .iter()
                    .map(|element| element.get_pixel(x, y).to_rgb()[2] as u32)
                    .sum::<u32>() as u32
                    / elements.len() as u32;
                img.put_pixel(x, y, Rgb([r_avg as u8, g_avg as u8, b_avg as u8]));
            }
        }

        img
    }
}

impl KmeansContext<RgbImage> for CharsetGeneratorContext {
    fn initialize_centroid(&self, _k: usize) -> RgbImage {
        let mut img = RgbImage::new(self.character_width, self.character_height);
        for x in 0..self.character_width {
            for y in 0..self.character_height {
                // TODO: support color/grayscale?
                let val = if random::<bool>() { 0xff } else { 0x00 };
                img.put_pixel(x, y, Rgb([val, val, val]));
            }
        }
        img
    }

    fn determine_centroid(&self, elements: &Vec<&RgbImage>) -> RgbImage {
        self.determine_colored_centroid(elements)
    }

    fn iteration_callback(&self, _i: u32, _clusters: &Vec<Cluster<RgbImage>>) -> bool {
        // println!("Iteration: {}", i);
        // self.draw_approximation(format!("iteration{}.png", i).as_str(), clusters);
        false
    }

    fn diff(&self, a: &RgbImage, b: &RgbImage) -> u32 {
        image_diff(a, b)
    }
}

pub struct ScreenProfile {
    pub lines: u32,
    pub columns: u32,
    pub character_width: u32,
    pub character_height: u32,
    pub chars: usize,
}

impl ScreenProfile {
    pub fn from_machine_profile(profile: &MachineProfile) -> Self {
        ScreenProfile {
            lines: profile.lines,
            columns: profile.columns,
            character_width: profile.charset_definition.character_width,
            character_height: profile.charset_definition.character_height,
            chars: 0x100,
        }
    }
}

fn initialize_tiles(profile: &ScreenProfile, input_img: &DynamicImage) -> Vec<RgbImage> {
    let mut tiles: Vec<RgbImage> = Vec::new();

    for row in 0..profile.lines {
        for column in 0..profile.columns {
            let tile = input_img.crop_imm(
                column * profile.character_width,
                row * profile.character_height,
                profile.character_width,
                profile.character_height,
            );
            tiles.push(tile.to_rgb8());
        }
    }

    tiles
}
