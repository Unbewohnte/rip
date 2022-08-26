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

const PNG_IDENTIFIER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0xD, 0xA, 0x1A, 0xA];
const PNG_END_IDENTIFIER: [u8; 8] = [0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

// Reads data from specified start_index position,
// if valid png bytes were found - returns exact positions of an image
pub fn rip_png(data: &[u8], start_index: usize) -> Option<Position> {
    if data.len() < PNG_IDENTIFIER.len() + PNG_END_IDENTIFIER.len() ||
        start_index + PNG_IDENTIFIER.len() + PNG_END_IDENTIFIER.len() > data.len() {
        return None;
    }

    let mut position: Position = Position{
        start: usize::MAX,
        end: usize::MAX,
        content_type: ContentType::PNG,
    };

    for i in start_index..data.len() {
        // start index
        if i < data.len() - PNG_IDENTIFIER.len() && position.start == usize::MAX {
            if data[i..i + PNG_IDENTIFIER.len()] == PNG_IDENTIFIER {
                position.start = i;
            }
        }

        // end index
        if i <= data.len() - PNG_END_IDENTIFIER.len() && position.end == usize::MAX {
            if data[i..i + PNG_END_IDENTIFIER.len()] == PNG_END_IDENTIFIER {
                position.end = i + PNG_END_IDENTIFIER.len();
                println!("end {}", position.end);
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