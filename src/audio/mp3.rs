/*
rip - rip embedded content
Copyright (C) 2022 Kasyanov Nikolay Alexeyevich (Unbewohnte)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::util::position::Position;
use crate::util::content_type::ContentType;

const ID3V2_IDENTIFIER: [u8; 3] = [0x49, 0x44, 0x33];
const ID3V2_HEADER_LENGTH: usize = 10;
const MP3_HEADER_LENGTH: usize = 4;
const MP3_HEADER_SYNC_WORD_MASK: u32 = 0xFFE00000;      // 11111111111000000000000000000000
const MP3_HEADER_VERSION_MASK: u32 = 0x00180000;        // 00000000000110000000000000000000
const MP3_HEADER_LAYER_MASK: u32 = 0x00060000;          // 00000000000001100000000000000000
const MP3_HEADER_BITRATE_MASK: u32 = 0x0000F000;        // 00000000000000001111000000000000
const MP3_HEADER_SAMPLING_RATE_MASK: u32 = 0x00000C00;  // 00000000000000000000110000000000
const MP3_HEADER_PADDING_MASK: u32 = 0x00000200;        // 00000000000000000000001000000000

// const MP3_HEADER_SYNC_WORD_MASK: u32 = 0xFFF00000;         // 11111111111100000000000000000000
// const MP3_HEADER_VERSION_MASK: u32 = 0xC0000;              // 00000000000011000000000000000000
// const MP3_HEADER_LAYER_MASK: u32 = 0x30000;                // 00000000000000110000000000000000
// const MP3_HEADER_BITRATE_MASK: u32 = 0x7800;               // 00000000000000000111100000000000
// const MP3_HEADER_SAMPLING_RATE_MASK: u32 = 0x600;          // 00000000000000000000011000000000
// const MP3_HEADER_PADDING_MASK: u32 = 0x100;                // 00000000000000000000000100000000

#[derive(Debug)]
enum AudioVersion {
    MpegV1,
    MpegV2,
    MpegV25,
}

#[derive(Debug)]
enum LayerIndex {
    LayerI,
    LayerII,
    LayerIII,
}

fn get_bitrate(header: u32, audio_version: &AudioVersion, layer: &LayerIndex) -> Option<u16> {
    match header & MP3_HEADER_BITRATE_MASK {
        // 0001
        0x1000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI | LayerIndex::LayerII | LayerIndex::LayerIII => return Some(32),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(32),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(8),
                    }
                }
            }
        }


        // 0010
        0x2000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(64),
                        LayerIndex::LayerII => return Some(48),
                        LayerIndex::LayerIII => return Some(40),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(48),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(16),
                    }
                }
            }
        }

        // 0011
        0x3000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(96),
                        LayerIndex::LayerII => return Some(56),
                        LayerIndex::LayerIII => return Some(48),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(56),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(24),
                    }
                }
            }
        }

        // 0100
        0x4000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(128),
                        LayerIndex::LayerII => return Some(64),
                        LayerIndex::LayerIII => return Some(56),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(64),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(32),
                    }
                }
            }
        }

        // 0101
        0x5000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(160),
                        LayerIndex::LayerII => return Some(80),
                        LayerIndex::LayerIII => return Some(64),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(80),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(40),
                    }
                }
            }
        }

        // 0110
        0x6000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(192),
                        LayerIndex::LayerII => return Some(96),
                        LayerIndex::LayerIII => return Some(80),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(96),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(48),
                    }
                }
            }
        }

        // 0111
        0x7000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(224),
                        LayerIndex::LayerII => return Some(112),
                        LayerIndex::LayerIII => return Some(96),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(112),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(56),
                    }
                }
            }
        }

        // 1000
        0x8000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(256),
                        LayerIndex::LayerII => return Some(128),
                        LayerIndex::LayerIII => return Some(112),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(128),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(64),
                    }
                }
            }
        }

        // 1001
        0x9000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(288),
                        LayerIndex::LayerII => return Some(160),
                        LayerIndex::LayerIII => return Some(128),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(144),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(80),
                    }
                }
            }
        }

        // 1010
        0xA000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(320),
                        LayerIndex::LayerII => return Some(192),
                        LayerIndex::LayerIII => return Some(160),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(160),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(96),
                    }
                }
            }
        }

        // 1011
        0xB000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(352),
                        LayerIndex::LayerII => return Some(224),
                        LayerIndex::LayerIII => return Some(192),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(176),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(112),
                    }
                }
            }
        }

        // 1100
        0xC000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(384),
                        LayerIndex::LayerII => return Some(256),
                        LayerIndex::LayerIII => return Some(224),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(192),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(128),
                    }
                }
            }
        }

        // 1101
        0xD000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(416),
                        LayerIndex::LayerII => return Some(320),
                        LayerIndex::LayerIII => return Some(256),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(224),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(144),
                    }
                }
            }
        }

        // 1110
        0xE000 => {
            match audio_version {
                AudioVersion::MpegV1 => {
                    match layer {
                        LayerIndex::LayerI => return Some(448),
                        LayerIndex::LayerII => return Some(384),
                        LayerIndex::LayerIII => return Some(320),
                    }
                }

                AudioVersion::MpegV2 | AudioVersion::MpegV25 => {
                    match layer {
                        LayerIndex::LayerI => return Some(256),
                        LayerIndex::LayerII | LayerIndex::LayerIII => return Some(160),
                    }
                }
            }
        }

        _ => {
            // invalid bitrate index
            return None;
        }
    }
}

fn get_samples_per_frame(audio_version: &AudioVersion, layer: &LayerIndex) -> u16 {
    match audio_version {
        AudioVersion::MpegV1 => {
            match layer {
                LayerIndex::LayerI => return 384,
                LayerIndex::LayerII => return 1152,
                LayerIndex::LayerIII => return 1152,
            }
        }

        AudioVersion::MpegV2 => {
            match layer {
                LayerIndex::LayerI => return 384,
                LayerIndex::LayerII => return 1152,
                LayerIndex::LayerIII => return 576,
            }
        }

        AudioVersion::MpegV25 => {
            match layer {
                LayerIndex::LayerI => return 384,
                LayerIndex::LayerII => return 1152,
                LayerIndex::LayerIII => return 576,
            }
        }
    }
}

fn get_sampling_rate(header: u32, audio_version: &AudioVersion) -> Option<u16> {
    match header & MP3_HEADER_SAMPLING_RATE_MASK {
        // 00
        0x0 => {
            match audio_version {
                AudioVersion::MpegV1 => return Some(44100),
                AudioVersion::MpegV2 => return Some(22050),
                AudioVersion::MpegV25 => return Some(11025),
            }
        }

        // 01
        0x400 => {
            match audio_version {
                AudioVersion::MpegV1 => return Some(48000),
                AudioVersion::MpegV2 => return Some(24000),
                AudioVersion::MpegV25 => return Some(12000),
            }
        }

        // 10
        0x800 => {
            match audio_version {
                AudioVersion::MpegV1 => return Some(32000),
                AudioVersion::MpegV2 => return Some(16000),
                AudioVersion::MpegV25 => return Some(8000),
            }
        }

        // invalid value
        _ => return None                  
    }
}

fn get_audio_version(header: u32) -> Option<AudioVersion> {
    match header & MP3_HEADER_VERSION_MASK {
        // 00
        0x0 => return Some(AudioVersion::MpegV25),
        // 10
        0x100000 => return Some(AudioVersion::MpegV2),
        // 11
        0x180000 => return Some(AudioVersion::MpegV1),
        // invalid version id
        _ => return None,
    }
}

fn get_layer(header: u32) -> Option<LayerIndex> {
    match header & MP3_HEADER_LAYER_MASK {
        // 01
        0x20000 => return Some(LayerIndex::LayerIII),
        // 10
        0x40000 => return Some(LayerIndex::LayerII),
        // 11
        0x60000 => return Some(LayerIndex::LayerI),
        // invalid layer index
        _ => return None,
    }
}

pub fn rip_mp3(data: &[u8], start_index: usize) -> Option<Position> {
    if data.len() < ID3V2_HEADER_LENGTH + MP3_HEADER_LENGTH + 1 ||
        start_index + ID3V2_HEADER_LENGTH + MP3_HEADER_LENGTH + 1 >= data.len() {
        return None;
    }

    let mut position: Position = Position{
        start: usize::MAX,
        end: usize::MAX,
        content_type: ContentType::MP3,
    };

    let mut cursor_index: usize;
    for i in start_index..data.len() {
        if i < ID3V2_HEADER_LENGTH && position.start == usize::MAX {
            if data[i..i + ID3V2_IDENTIFIER.len()] == ID3V2_IDENTIFIER {
                // found ID3v2 tag (the beginning of the MP3 file)
                println!("id3 at {}", i);

                // get tag length
                let mut tag_length_bytes: [u8; 4] = [0; 4];
                for j in 0..4 {
                    tag_length_bytes[j] = data[i+ID3V2_IDENTIFIER.len()+3+j];
                }
                // convert syncsafe integer to a normal one
                let mut tag_length: u32 = 0;
                for j in 0..4 {
                    tag_length = tag_length << 7;
                    tag_length = tag_length | tag_length_bytes[j] as u32;
                }

                let id3v2_end_index: usize = i + ID3V2_HEADER_LENGTH + tag_length as usize;
                println!("id3v2 end index is {}; the data size is {}", id3v2_end_index, data.len());
                if id3v2_end_index + MP3_HEADER_LENGTH > data.len() - 1 {
                    println!("not enough data length");
                    // strange: there's a valid ID3 tag but not enough data to store any music
                    break;
                }
                position.start = i;

                cursor_index = id3v2_end_index;
                loop {
                    if cursor_index >= data.len() - 1 {
                        break;
                    }

                    // whole header
                    let mut mp3_header_bytes: [u8 ; MP3_HEADER_LENGTH] = [0; MP3_HEADER_LENGTH];
                    for j in 0..MP3_HEADER_LENGTH {
                        mp3_header_bytes[j] = data[cursor_index + j];
                    }
                    let mp3_header: u32 = u32::from_be_bytes(mp3_header_bytes);

                    // check for sync word
                    if !mp3_header & MP3_HEADER_SYNC_WORD_MASK == MP3_HEADER_SYNC_WORD_MASK {
                        println!("SYNCWORD NO");
                        break;
                    }

                    // that's really an MP3 !
                    println!("mpeg header at {}", cursor_index);
                    cursor_index += MP3_HEADER_LENGTH;

                    println!("{:#032b}", mp3_header);

                    // retrieve audio version
                    let audio_version: AudioVersion;
                    match get_audio_version(mp3_header) {
                        Some(version) => audio_version = version,
                        None => {println!("VERSION NO {:032x}", mp3_header & MP3_HEADER_VERSION_MASK); break},
                    }

                    println!("audio version is {:?}", audio_version);

                    // get layer
                    let layer: LayerIndex;
                    match get_layer(mp3_header) {
                        Some(l) => layer = l,
                        None => {println!("LAYER NO"); break},
                    }

                    println!("layer is {:?}", layer);


                    // decode that HUGE bitrate table
                    let bitrate: u16;
                    match get_bitrate(mp3_header, &audio_version, &layer) {
                        Some(rate) => bitrate = rate,
                        None => {println!("BITRATE NO"); break},
                    }

                    println!("bitrate is {}", bitrate);


                    // samples per frame
                    // let samples_per_frame: u16 = get_samples_per_frame(&audio_version, &layer);

                    // sampling rate
                    let sampling_rate: u16;
                    match get_sampling_rate(mp3_header, &audio_version) {
                        Some(rate) => sampling_rate = rate,
                        None => {println!("SAMPLING RATE NO"); break},
                    }

                    println!("sampling rate is {}", sampling_rate);

                
                    // padding
                    let padding: u8;
                    if mp3_header == MP3_HEADER_PADDING_MASK {
                        padding = 1;
                    } else {
                        padding = 0; 
                    }

                    println!("padding is {}", padding);

                    let slot_size: u32;
                    match layer {
                        LayerIndex::LayerI => slot_size = 4,
                        _ => slot_size = 1,
                    }

                    let multiplier: u32;
                    match layer {
                        LayerIndex::LayerI => multiplier = 12,
                        _ => multiplier = 144000,
                    }

                    let slot_count: u32 = ((multiplier as u32 * (bitrate as u32 * 1000)) / sampling_rate as u32) + padding as u32;

                    // finally calculate frame size
                    let frame_size: u32 = slot_count * slot_size;
                    println!("frame size is {}", frame_size);

                    // set cursor to the next frame
                    cursor_index += frame_size as usize;
                    // extend end position
                    position.end = cursor_index;
                    println!("frame end at {}", cursor_index);
                }
            }
        }

        if position.start != usize::MAX && position.end != usize::MAX {
            break;
        }
    }   

    if position.start == usize::MAX || position.end == usize::MAX || position.end <= position.start {
        return None;
    }

    return Some(position);
}