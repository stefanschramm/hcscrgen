use crate::Character;

pub struct MachineProfile {
    pub lines: u32,
    pub columns: u32,
    pub character_ram_mapping: fn(character: &Character) -> u8,
    pub color_ram_mapping: Option<fn(character: &Character) -> u8>,
    pub charset_definition: CharsetDefinition,
    // TODO: static reference to static vector of include_bytes instead for charset definitions?
    pub charsets: &'static [&'static str],
}

pub struct CharsetDefinition {
    pub mode: MatrixCharsetOrder,
    pub character_width: u32,
    pub character_height: u32,
    pub offset_top: u32,
    pub spacing_vertical: u32,
    pub offset_left: u32,
    pub spacing_horizontal: u32,
}

pub enum MatrixCharsetOrder {
    /// top to bottom first
    RowInLowNibble,
    /// left to right first
    ColumnInLowNibble,
}

/// KC 87 profile
///
/// Character RAM: 0xec00
/// Color RAM: 0xe800
///
/// https://hc-ddr.hucki.net/wiki/doku.php/z9001/versionen
pub const KC87_PROFILE: MachineProfile = MachineProfile {
    lines: 24,
    columns: 40,
    character_ram_mapping: |character| character.code,
    color_ram_mapping: Some(|_character| 0b01110000),
    charset_definition: CharsetDefinition {
        mode: MatrixCharsetOrder::ColumnInLowNibble,
        character_width: 8,
        character_height: 8,
        offset_top: 1,
        spacing_vertical: 1,
        offset_left: 1,
        spacing_horizontal: 1,
    },
    charsets: &["profiles/kc87/charset_inverted.png"],
};

/// Sharp MZ profile
///
/// Character RAM: 0xd000
/// Color RAM: 0xd800
///
/// https://original.sharpmz.org/mz-700/colorvram.htm
/// https://original.sharpmz.org/mz-700/codetable.htm
pub const SHARPMZ_PROFILE: MachineProfile = MachineProfile {
    lines: 25,
    columns: 40,
    character_ram_mapping: |character| character.code,
    color_ram_mapping: Some(|character| if character.charset == 0 { 0x07 } else { 0x87 }),
    charset_definition: CharsetDefinition {
        mode: MatrixCharsetOrder::RowInLowNibble,
        character_width: 8,
        character_height: 8,
        offset_top: 2,
        spacing_vertical: 3,
        offset_left: 2,
        spacing_horizontal: 3,
    },
    charsets: &[
        "profiles/sharp_mz-700/charset.png",
        "profiles/sharp_mz-700/charset_extended.png",
    ],
};

/// C64 profile
///
/// Character RAM: 0x0400
/// Color RAM: 0xd800
///
/// https://www.c64-wiki.com/wiki/Color_RAM
/// https://www.c64-wiki.com/wiki/Color
pub const C64_PROFILE: MachineProfile = MachineProfile {
    lines: 25,
    columns: 40,
    character_ram_mapping: |character| character.code,
    color_ram_mapping: Some(|_character| 0x01),
    charset_definition: CharsetDefinition {
        mode: MatrixCharsetOrder::RowInLowNibble,
        character_width: 8,
        character_height: 8,
        offset_top: 0,
        spacing_vertical: 0,
        offset_left: 0,
        spacing_horizontal: 0,
    },
    charsets: &["profiles/c64/C64_Petscii_Charts.png"],
};

/// Z 1013 Profile
///
/// Character RAM: 0xec00
///
/// https://hc-ddr.hucki.net/wiki/doku.php/z1013/erweiterungen/zeichensatz
pub const Z1013_PROFILE: MachineProfile = MachineProfile {
    lines: 32,
    columns: 32,
    character_ram_mapping: |character| character.code,
    color_ram_mapping: None,
    charset_definition: CharsetDefinition {
        mode: MatrixCharsetOrder::ColumnInLowNibble,
        character_width: 8,
        character_height: 8,
        offset_top: 1,
        spacing_vertical: 1,
        offset_left: 1,
        spacing_horizontal: 1,
    },
    charsets: &["profiles/z1013/zg_1013_orig_inverted.png"],
};
