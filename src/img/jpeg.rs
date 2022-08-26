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

const JPEG_IDENTIFIER: [u8; 3] = [0xFF, 0xD8, 0xFF];
const JPEG_END_IDENTIFIER: [u8; 2] = [0xFF, 0xD9];

// Reads data from specified start_index position,
// if valid png bytes were found - returns exact positions of an image
pub fn rip_jpeg(data: &[u8], start_index: usize) -> Option<Position> {
    if data.len() < JPEG_IDENTIFIER.len() + JPEG_END_IDENTIFIER.len() ||
        start_index + JPEG_IDENTIFIER.len() + JPEG_END_IDENTIFIER.len() > data.len() {
        return None;
    }

    let mut position: Position = Position{
        start: usize::MAX,
        end: usize::MAX,
        content_type: ContentType::JPEG,
    };

    for i in start_index..data.len() {
        // start index
        if i < data.len() - JPEG_IDENTIFIER.len() && position.start == usize::MAX {
            if data[i..i + JPEG_IDENTIFIER.len()] == JPEG_IDENTIFIER {
                position.start = i;
            }
        }

        // end index
        if i <= data.len() - JPEG_END_IDENTIFIER.len() && position.end == usize::MAX {
            if data[i..i + JPEG_END_IDENTIFIER.len()] == JPEG_END_IDENTIFIER {
                position.end = i;
            }
        }

        if position.start != usize::MAX && position.end != usize::MAX {
            break;
        }
    }

    if position.start == usize::MAX || position.end == usize::MAX || position.end < position.start {
        return None;
    }

    return Some(position);
}