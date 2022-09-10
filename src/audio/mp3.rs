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

// bitrate table for mpeg Version+Layer
const MP3_BITRATE_TABLE: [[[u32; 15]; 3]; 2] = [
    // mpegv1
    [
        // layer I
        [
            0, 32000, 64000, 96000, 128000, 160000,
            192000, 224000, 256000, 288000, 320000,
            352000, 384000, 416000, 448000,
        ],
        // layer II
        [
            0, 32000, 48000, 56000, 64000, 80000,
            96000, 112000, 128000, 160000, 192000,
            224000, 256000, 320000, 384000,
        ],
        // layer III
        [
            0, 32000, 40000, 48000, 56000, 64000,
            80000, 96000, 112000, 128000, 160000,
            192000, 224000, 256000, 320000,
        ],
    ],
    //mpegv2
    [
        // layer I
        [
            0, 32_000, 48_000, 56_000, 64_000, 80_000,
            96_000, 112_000, 128_000, 144_000, 160_000,
            176_000, 192_000, 224_000, 256_000,
        ],
        // layer III
        [
            0, 8_000, 16_000, 24_000, 32_000, 40_000, 48_000,
            56_000, 64_000, 80_000, 96_000, 112_000, 128_000,
            144_000, 160_000,
        ],
        // layer III
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    ],
];

enum Layer {
    I,
    II,
    III,
}

#[derive(Debug, PartialEq, Eq)]
enum MpegVersion {
    V1,
    V2,
    V2_5,
}

struct MP3Header {
    bitrate: u32,
    layer: Layer,
    version: MpegVersion,
    sampling_rate: u32,
    padding: bool,
}

impl MP3Header {
    fn from_bytes(header_bytes: &[u8; MP3_HEADER_LENGTH]) -> Result<MP3Header, &'static str> {
        let header: u32 = u32::from_be_bytes(*header_bytes);
        // check sync
        if header & 0xFFE00000 != 0xFFE00000 {
            return Err("does not contain sync");
        }

        // get version
        let version: MpegVersion;
        match (header & 0x180000) >> 19 {
            0b00 => version = MpegVersion::V2_5,
            0b10 => version = MpegVersion::V2,
            0b11 => version = MpegVersion::V1,
            _ => return Err("invalid mpeg version"),
        }

        // get layer
        let layer: Layer;
        match (header & 0x60000) >> 17 {
            0b01 => layer = Layer::III,
            0b10 => layer = Layer::II,
            0b11 => layer = Layer::I,
            _ => return Err("invalid mpeg layer"),
        }

        // calculate bitrate
        let bitrate: u32;
        match ((header & 0xF000) >> 12, &version, &layer) {
            (n, MpegVersion::V1, Layer::I) => bitrate = MP3_BITRATE_TABLE[0][0][n as usize],
            (n, MpegVersion::V1, Layer::II) => bitrate = MP3_BITRATE_TABLE[0][1][n as usize],
            (n, MpegVersion::V1, Layer::III) => bitrate = MP3_BITRATE_TABLE[0][2][n as usize],
            (n, MpegVersion::V2|MpegVersion::V2_5, Layer::I) => bitrate = MP3_BITRATE_TABLE[1][0][n as usize],
            (n, MpegVersion::V2|MpegVersion::V2_5, Layer::III) => bitrate = MP3_BITRATE_TABLE[1][1][n as usize],
            _ => return Err("invalid or too tricky frame header to calculate bitrate"),
        }

        // sample rate
        let sampling_rate: u32;
        match ((header & 0xC00) >> 10, &version) {
            (0b00, MpegVersion::V1) => sampling_rate = 44100,
            (0b01, MpegVersion::V1) => sampling_rate = 48000,
            (0b10, MpegVersion::V1) => sampling_rate = 32000,
            (0b00, MpegVersion::V2) => sampling_rate = 22050,
            (0b01, MpegVersion::V2) => sampling_rate = 24000,
            (0b10, MpegVersion::V2) => sampling_rate = 16000,
            (0b00, MpegVersion::V2_5) => sampling_rate = 11025,
            (0b01, MpegVersion::V2_5) => sampling_rate = 12000,
            (0b10, MpegVersion::V2_5) => sampling_rate = 8000,
            _ => return Err("invalid sampling rate calculation"),
        }

        let padding: bool = header & 0x200 != 0;

        return Ok(MP3Header{
            bitrate: bitrate,
            layer: layer,
            version: version,
            sampling_rate: sampling_rate,
            padding: padding,
        });
    }

    fn frame_size(&self) -> usize {
        return {
            (if self.version == MpegVersion::V1 {144} else {72} *
             self.bitrate /
             self.sampling_rate) as usize + if self.padding {1} else {0} 
        };
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

    for i in start_index..data.len() {
        if i < ID3V2_HEADER_LENGTH && position.start == usize::MAX {
            if data[i..i + ID3V2_IDENTIFIER.len()] == ID3V2_IDENTIFIER {
                // found ID3v2 tag (the beginning of the MP3 file)
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
                if id3v2_end_index + MP3_HEADER_LENGTH > data.len() - 1 {
                    println!("not enough data length");
                    // strange: there's a valid ID3 tag but not enough data to store any music
                    break;
                }
                position.start = i;

                position.end = id3v2_end_index;
                break;
            }
        }
    }

    // return None immediately if id3 tag was not found
    if position.start == usize::MAX {
        return None;
    }

    // try to extract mp3 frames
    let mut mp3_header_bytes: [u8; MP3_HEADER_LENGTH] = [0; MP3_HEADER_LENGTH];
    while position.end < data.len() - MP3_HEADER_LENGTH {
        for j in 0..MP3_HEADER_LENGTH {
            mp3_header_bytes[j] = data[position.end + j];
        }

        match MP3Header::from_bytes(&mp3_header_bytes) {
            Ok(header) => {
                position.end += header.frame_size();
            }
            Err(_) => {
                break;
            }
        }
    }

    if position.start == usize::MAX || position.end == usize::MAX || position.end <= position.start {
        return None;
    }

    return Some(position);
}