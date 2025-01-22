use charset::{convert_charset, generate_charset, ScreenProfile};
use image::{DynamicImage, GenericImage, RgbImage};
use profiles::{Character, MachineProfile, AVAILABLE_PROFILES};
use utils::{image_diff, load_charset};

mod charset;
mod profiles;
mod utils;

pub struct ConversionResult {
    pub preview: RgbImage,
    pub character_ram: Vec<u8>,
    pub color_ram: Option<Vec<u8>>,
    pub charset: Option<Vec<u8>>,
}

pub fn convert(
    input_img: &DynamicImage,
    profile_identifier: &str,
    autogenerate_charset: bool,
) -> Result<ConversionResult, String> {
    for profile in AVAILABLE_PROFILES {
        if profile.identifier == profile_identifier {
            let charset = if autogenerate_charset {
                let charset =
                    generate_charset(&ScreenProfile::from_machine_profile(profile), input_img);
                Some(charset)
            } else {
                None
            };

            return Converter::new(profile, charset).convert(input_img);
        }
    }

    let identifiers = AVAILABLE_PROFILES
        .iter()
        .map(|p| p.identifier)
        .collect::<Vec<&str>>()
        .join(", ");

    return Err(format!(
        "Unknown profile identifier \"{}\".\nAvailable profiles: {}",
        profile_identifier, identifiers
    ));
}

struct Converter<'a> {
    profile: &'a MachineProfile,
    charsets: Vec<Vec<RgbImage>>,
    uses_custom_charset: bool,
    screen_height: u32,
    screen_width: u32,
}

impl<'a> Converter<'a> {
    fn new(profile: &'a MachineProfile, custom_charset: Option<Vec<RgbImage>>) -> Self {
        let uses_custom_charset = custom_charset.is_some();
        let charsets = {
            if let Some(custom_charset) = custom_charset {
                vec![custom_charset]
            } else {
                let mut charsets = Vec::new();

                for charset_file in profile.charsets {
                    let charset = load_charset(charset_file, &profile.charset_definition);
                    charsets.push(charset);
                }

                charsets
            }
        };

        Self {
            charsets,
            uses_custom_charset,
            screen_height: profile.lines * profile.charset_definition.character_height,
            screen_width: profile.columns * profile.charset_definition.character_width,
            profile: profile,
        }
    }

    fn convert(&self, input_img: &DynamicImage) -> Result<crate::ConversionResult, String> {
        if input_img.width() < self.screen_width || input_img.height() < self.screen_height {
            return Err(format!(
                "Input file must have a dimension of at least {}x{} pixels.",
                self.screen_width, self.screen_height
            ));
        }

        let mut characters: Vec<Character> = Vec::new();

        for row in 0..self.profile.lines {
            for column in 0..self.profile.columns {
                let tile = input_img
                    .crop_imm(
                        column * self.profile.charset_definition.character_width,
                        row * self.profile.charset_definition.character_height,
                        self.profile.charset_definition.character_width,
                        self.profile.charset_definition.character_height,
                    )
                    .into_rgb8();
                characters.push(self.get_best_matching_character(&tile));
            }
        }

        Ok(ConversionResult {
            preview: self.create_preview(&characters),
            character_ram: self.map_character_ram(&characters),
            color_ram: self.map_color_ram(&characters),
            charset: if self.uses_custom_charset {
                Some(convert_charset(&self.charsets[0]))
            } else {
                None
            },
        })
    }

    fn get_best_matching_character(&self, tile: &RgbImage) -> Character {
        let mut best_character = Character {
            charset: 0,
            code: 0,
        };
        let mut best_diff = u32::MAX;
        for (charset, characters) in self.charsets.iter().enumerate() {
            for (code, character) in characters.iter().enumerate() {
                let diff = image_diff(&tile, character);
                if diff < best_diff {
                    best_character = Character {
                        charset: charset as u32,
                        code: code as u8,
                    };
                    best_diff = diff;
                }
            }
        }

        best_character
    }

    fn create_preview(&self, characters: &Vec<Character>) -> RgbImage {
        let mut preview_img = RgbImage::new(self.screen_width, self.screen_height);
        for (i, character) in characters.iter().enumerate() {
            let row = i as u32 / self.profile.columns;
            let column = i as u32 - row * self.profile.columns;

            preview_img
                .copy_from(
                    &self.charsets[character.charset as usize][character.code as usize],
                    column * self.profile.charset_definition.character_width,
                    row * self.profile.charset_definition.character_height,
                )
                .expect("Unable to put tile into preview image");
        }

        preview_img
    }

    fn map_character_ram(&self, characters: &Vec<Character>) -> Vec<u8> {
        characters
            .into_iter()
            .map(self.profile.character_ram_mapping)
            .collect()
    }

    fn map_color_ram(&self, characters: &Vec<Character>) -> Option<Vec<u8>> {
        match self.profile.color_ram_mapping {
            Some(mapping) => Some(characters.iter().map(mapping).collect()),
            None => None,
        }
    }
}
