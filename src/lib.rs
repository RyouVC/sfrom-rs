use nom::{
    bytes::complete::{tag, take},
    number::complete::{le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult,
};
use std::io::{self, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct SfromHeader {
    pub magic: u32, // 0x00000100
    pub file_size: u32,
    pub rom_location: u32, // Usually 0x30
    pub pcm_samples_location: u32,
    pub pcm_footer_location: u32,
    pub footer_location: u32,
    pub sdd1_data_offset: u32,
    pub reserved1: u32, // 0x00000000
    //
    pub unknown1: u32,
    // 0x8
    pub wiiu_game_id: [u8; 8],
    pub reserved2: u32, // 0x00000000
}

#[derive(Debug)]
pub struct SfromFooter {
    pub fps: u8, // 0x3C = 60fps, 0x32 = 50fps
    pub rom_size: u32,
    pub pcm_samples_size: u32,
    pub pcm_footer_size: u32,
    pub preset_id: u16,
    pub player_count: u8,
    pub sound_volume: u8,
    pub rom_type: u8, // 0x14 = LoROM, 0x15 = HiROM
    pub enhancement_chip: u8,
    pub unknown1: u32, // Usually 0x1
    pub unknown2: u32, // Always 0x1
}

#[derive(Debug)]
pub struct GameTagData {
    /// Threshold for Armet,
    /// the Epilepsy reduction filter
    pub armet_threshold: Option<[u8; 3]>, // Tag 'A'
    pub sdd1_data: Option<Vec<u8>>,       // Tag 'D'
    pub preset_id: Option<u16>,           // Tag 'G'
    pub flags: Option<[u8; 7]>,           // Tag 'P'
    pub unknown_s: Option<[u8; 3]>,       // Tag 'S'
    pub superfx_clock: Option<u16>,       // Tag 'U'
    pub armet_version: Option<u8>,        // Tag 'a'
    pub snes_header_location: Option<u8>, // Tag 'c'
    pub unknown_d: Option<u8>,            // Tag 'd'
    pub enhancement_chip: Option<u8>,     // Tag 'e'
    pub resolution_ratio: Option<u8>,     // Tag 'h'
    pub unknown_j: Option<u8>,            // Tag 'j'
    pub mouse_flag: Option<u8>,           // Tag 'm'
    pub max_players: Option<u8>,          // Tag 'p'
    pub visible_height: Option<u8>,       // Tag 'r'
    pub unknown_t: Option<u8>,            // Tag 't'
    pub volume: Option<u8>,               // Tag 'v'
}

#[repr(u8)]
pub enum EnhancementChip {
    Normal = 0x00,
    Dsp1 = 0x02,
    Sdd1 = 0x03,
    Cx4 = 0x04,
    MegaManX = 0x05, // Copy protection fix?
    Sa1_1 = 0x06,
    Sa1_2 = 0x07,
    Sa1_3 = 0x08,
    Sa1_4 = 0x09,
    Sa1_5 = 0x0A,
    Sa1_6 = 0x0B,
    SuperFx = 0x0C,
}

impl SfromHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (
            input,
            (
                magic,
                file_size,
                rom_location,
                pcm_samples_location,
                pcm_footer_location,
                footer_location,
                sdd1_data_offset,
                reserved1,
                unknown1,
                wiiu_game_id,
                reserved2,
            ),
        ) = tuple((
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            take(8usize),
            le_u32,
        ))(input)?;

        Ok((
            input,
            SfromHeader {
                magic,
                file_size,
                rom_location,
                pcm_samples_location,
                pcm_footer_location,
                footer_location,
                sdd1_data_offset,
                reserved1,
                unknown1,
                wiiu_game_id: wiiu_game_id.try_into().unwrap(),
                reserved2,
            },
        ))
    }
}

impl SfromFooter {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (
            input,
            (
                fps,
                rom_size,
                pcm_samples_size,
                pcm_footer_size,
                preset_id,
                player_count,
                sound_volume,
                rom_type,
                enhancement_chip,
                unknown1,
                unknown2,
            ),
        ) = tuple((
            le_u8, le_u32, le_u32, le_u32, le_u16, le_u8, le_u8, le_u8, le_u8, le_u32, le_u32,
        ))(input)?;

        Ok((
            input,
            SfromFooter {
                fps,
                rom_size,
                pcm_samples_size,
                pcm_footer_size,
                preset_id,
                player_count,
                sound_volume,
                rom_type,
                enhancement_chip,
                unknown1,
                unknown2,
            },
        ))
    }
}

impl GameTagData {
    pub fn parse(input: &[u8]) -> Self {
        let mut data = GameTagData {
            armet_threshold: None,
            sdd1_data: None,
            preset_id: None,
            flags: None,
            unknown_s: None,
            superfx_clock: None,
            armet_version: None,
            snes_header_location: None,
            unknown_d: None,
            enhancement_chip: None,
            resolution_ratio: None,
            unknown_j: None,
            mouse_flag: None,
            max_players: None,
            visible_height: None,
            unknown_t: None,
            volume: None,
        };

        let mut i = 0;
        while i < input.len() {
            match input[i] {
                0x41 => {
                    // 'A'
                    if i + 3 < input.len() {
                        data.armet_threshold = Some([input[i + 1], input[i + 2], input[i + 3]]);
                    }
                    i += 4;
                }
                0x44 => {
                    // 'D'
                    if i + 4 < input.len() {
                        let size =
                            u32::from_le_bytes([input[i + 1], input[i + 2], input[i + 3], 0]);
                        if i + 4 + size as usize <= input.len() {
                            data.sdd1_data = Some(input[i + 4..i + 4 + size as usize].to_vec());
                        }
                        i += 4 + size as usize;
                    } else {
                        i += 1;
                    }
                }
                0x47 => {
                    // 'G'
                    if i + 5 < input.len() {
                        data.preset_id = Some(u16::from_le_bytes([input[i + 3], input[i + 4]]));
                    }
                    i += 5;
                }
                0x50 => {
                    // 'P'
                    if i + 7 < input.len() {
                        data.flags = Some([
                            input[i + 1],
                            input[i + 2],
                            input[i + 3],
                            input[i + 4],
                            input[i + 5],
                            input[i + 6],
                            input[i + 7],
                        ]);
                    }
                    i += 7;
                }
                0x53 => {
                    // 'S'
                    if i + 3 < input.len() {
                        data.unknown_s = Some([input[i + 1], input[i + 2], input[i + 3]]);
                    }
                    i += 3;
                }
                0x55 => {
                    // 'U'
                    if i + 5 < input.len() {
                        data.superfx_clock = Some(u16::from_le_bytes([input[i + 3], input[i + 4]]));
                    }
                    i += 5;
                }
                0x61 => {
                    data.armet_version = Some(input[i + 1]);
                    i += 1;
                } // 'a'
                0x63 => {
                    data.snes_header_location = Some(input[i + 1]);
                    i += 1;
                } // 'c'
                0x64 => {
                    data.unknown_d = Some(input[i + 1]);
                    i += 1;
                } // 'd'
                0x65 => {
                    data.enhancement_chip = Some(input[i + 1]);
                    i += 1;
                } // 'e'
                0x68 => {
                    data.resolution_ratio = Some(input[i + 1]);
                    i += 1;
                } // 'h'
                0x6A => {
                    data.unknown_j = Some(input[i + 1]);
                    i += 1;
                } // 'j'
                0x6D => {
                    data.mouse_flag = Some(input[i + 1]);
                    i += 1;
                } // 'm'
                0x70 => {
                    data.max_players = Some(input[i + 1]);
                    i += 1;
                } // 'p'
                0x72 => {
                    data.visible_height = Some(input[i + 1]);
                    i += 1;
                } // 'r'
                0x74 => {
                    data.unknown_t = Some(input[i + 1]);
                    i += 1;
                } // 't'
                0x76 => {
                    data.volume = Some(input[i + 1]);
                    i += 1;
                } // 'v'
                _ => i += 1,
            }
        }

        data
    }
}

#[derive(Debug)]
pub struct SwitchSfromFooter {
    pub game_tags: GameTagData,
    pub game_info_size: u32,
    pub magic: [u8; 4], // "Can1"
}

// Helper function for 24-bit little endian integers
fn le_u24(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, bytes) = take(3usize)(input)?;
    Ok((
        input,
        bytes[0] as u32 | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16),
    ))
}
#[derive(Debug)]
pub enum SfromType {
    Classic(SfromHeader),
    NintendoSwitch(SwitchSfromFooter),
}

impl SfromType {
    // check for the first 4 bytes for the magic header of
    // 0x00000100
    // If exists, is a Classic type.
    //
    // If not and it contains a normal SFC magic byte, it's
    // an NSO ROM
    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        // get the first 4 bytes
        let (_input_discard, bytes) = take(4usize)(input)?;
        //println!("First 4 bytes: {:X?}", bytes);
        if bytes == [0x00, 0x01, 0x00, 0x00] {
            println!("Detected SNES Mini/Classic SFROM, parsing header...");
            let (input, header) = SfromHeader::parse(input)?;
            Ok((input, Self::Classic(header)))
        } else {
            // Check last 4 bytes are Can1
            let last_bytes = &input[input.len() - 4..];
            if last_bytes == [0x43, 0x61, 0x6E, 0x31] {
                println!("Detected Switch SFROM, parsing footer...");

                // Get Game Info Data size (4 bytes before Can1)
                let info_size_bytes = &input[input.len() - 8..input.len() - 4];
                let info_size = u32::from_le_bytes(info_size_bytes.try_into().unwrap());

                // Game Info Data starts at: input.len() - 8 - info_size
                let info_start = input.len() - 8 - info_size as usize;
                let info_data = input[info_start..info_start + info_size as usize].to_vec();

                let footer = SwitchSfromFooter {
                    game_tags: GameTagData::parse(&info_data),
                    game_info_size: info_size,
                    magic: [0x43, 0x61, 0x6E, 0x31], // "Can1"
                };

                Ok((input, Self::NintendoSwitch(footer)))
            } else {
                println!("Unknown file detected");
                panic!("Unknown file format");
            }
        }
    }
}
#[derive(Debug)]
pub struct Sfrom {
    pub header_type: SfromType,
    pub rom_data: Vec<u8>,
    pub footer: Option<SfromFooter>,
}

impl Sfrom {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (_, header_type) = SfromType::from_bytes(input)?;
        match header_type {
            SfromType::Classic(ref header) => {
                // Get the starting location of the ROM data
                let rom_start = header.rom_location as usize;

                let pcm_start = header.pcm_samples_location as usize;
                let rom_data = input[rom_start..pcm_start].to_vec();
                let pcm_data_end = header.pcm_footer_location as usize;

                let pcm_data = input[pcm_start..pcm_data_end].to_vec();
                let footer_start = header.footer_location as usize;

                let pcm_footer_data = input[pcm_data_end..footer_start].to_vec();

                let footer_data = input[footer_start..].to_vec();

                // let footer_data = input[footer_start..].to_vec();

                Ok((
                    input,
                    Self {
                        header_type,
                        rom_data,
                        footer: None,
                    },
                ))
            }
            SfromType::NintendoSwitch(footer) => {
                let rom_data = input[..input.len() - 8].to_vec();
                Ok((
                    input,
                    Self {
                        header_type: SfromType::NintendoSwitch(footer),
                        rom_data,
                        footer: None,
                    },
                ))
            }
        }
    }
}

// SFROM parser is kinda weird and stuff but for now i just bother with the SFC converter

#[derive(Debug)]
pub struct Sfc {
    pub rom_data: Vec<u8>,
}
impl Sfc {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let rom_data = input.to_vec();
        Ok((input, Self { rom_data }))
    }

    pub fn convert_to_switch(&self) -> Vec<u8> {
        let mut modified_rom_data = self.rom_data.clone();
        let mut footer = [
            0x47, 0x02, 0x00, 0x00, 0x11, 0x10, 0x76, 0x91, 0x74, 0x06, 0x70, 0x02, 0x0C, 0x00,
            0x00, 0x00, 0x43, 0x61, 0x6E, 0x31,
        ];
        modified_rom_data.extend_from_slice(&footer);
        modified_rom_data
    }

    pub fn convert_to_snes_mini(&self) -> Vec<u8> {
        let modified_rom_data = self.rom_data.clone();
        modified_rom_data
    }
}
