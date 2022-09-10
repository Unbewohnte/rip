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

mod util;
mod img;
mod audio;

use std::path;
use std::io::{Read, Write};
use crate::util::position::Position;
use crate::util::content_type::ContentType;
use crate::img::png::rip_png;
use crate::img::jpeg::rip_jpeg;
use crate::audio::mp3::rip_mp3;

#[derive(Debug)]
enum RipType {
    ALL,
    IMG,
    AUDIO,
}

fn main() {
    let mut save_directory: &path::Path = path::Path::new(".");
    let mut file_paths: Vec<&path::Path> = Vec::new();
    let mut max_file_size: u128 = u128::MAX;
    let mut rip_type: RipType = RipType::ALL;

    // work out the arguments
    let args: Vec<String> = std::env::args().collect();
    let argc: usize = args.len();

    if argc < 2 {
        println!("[INFO] Not enough arguments. Run rip --help to get the insight on the usage");
        return;
    }

    let mut arg_index: usize = 1;
    while arg_index < argc {
        if &args[arg_index] == "-h" || &args[arg_index] == "--help" {
            println!(
                "rip (optional)[FLAG]... (optional; default=ALL)[RIPTYPE] [FILE]...\
                \n
                \n\
                [FLAG]s\n\
                \"-v\"   or \"--version\"               -> print version\n\
                \"-h\"   or \"--help\"                  -> print this message\n\
                \"-sd\"  or \"--save-dir\" [DIR]        -> specify save directory\n\
                \"-mfs\" or \"--max-file-size\" [SIZE]  -> skip files bigger than size (in bytes)\n
                \n\
                [RIPTYPE]\n\
                ALL   -> rip everything that seems like an embedded content\n\
                IMG   -> try to look for images only\n\
                AUDIO -> rip audio content"
            );
            return;
        }
        else if &args[arg_index] == "-v" || &args[arg_index] == "--version" {
            println!(
                "rip v0.3\n\n\
                (c) 2022 Kasyanov Nikolay Alexeyevich (Unbewohnte)"
            );
            return;
        }
        else if &args[arg_index] == "-sd" || &args[arg_index] == "--save-dir" {
            if arg_index + 1 >= argc {
                println!("[ERROR] Not enough arguments to set save directory and launch RIP");
                return;
            }

            arg_index += 1; // increment index no matter the further outcome
            let specified_save_dir: &path::Path = path::Path::new(&args[arg_index]);
            if !specified_save_dir.exists() {
                // does not exist
                match std::fs::create_dir_all(specified_save_dir) {
                    Ok(()) => {
                        save_directory = specified_save_dir;
                    }

                    Err(error) => {
                        save_directory = &path::Path::new(".");
                        println!("[ERROR] Error creating specified save directory: {}. Using working dir instead...", error);
                    }
                }
            }
            else if !specified_save_dir.is_dir() {
                // it exists, but not a directory
                println!("[ERROR] Specified save directory \"{}\" is NOT a directory. Using working dir instead...", specified_save_dir.display());
                save_directory = &path::Path::new(".");
            }
            else {
                // exists and IS directory ! Everything's okay and easy
                save_directory = specified_save_dir;
            }

        }
        else if &args[arg_index] == "-mfs" || &args[arg_index] == "--max-file-size" {
            if arg_index + 1 >= argc {
                println!("[ERROR] Not enough arguments to set max file size and launch RIP");
                return;
            }

            arg_index += 1;
            match args[arg_index].parse::<u128>() {
                Ok(max_fsize) => {
                    max_file_size = max_fsize;
                }

                Err(_) => {
                    println!("[ERROR] Invalid max file size was specified");
                    return;
                }
            }
        }
        else if file_paths.len() == 0 && &args[arg_index].to_lowercase() == "all" {
            rip_type = RipType::ALL;
            println!("Ripping EVERYTHING (JPEG, PNG, MP3)");
        }
        else if file_paths.len() == 0 && &args[arg_index].to_lowercase() == "img" {
            rip_type = RipType::IMG;
            println!("Ripping IMAGES (JPEG, PNG)");
        }
        else if file_paths.len() == 0 && &args[arg_index].to_lowercase() == "audio" {
            rip_type = RipType::AUDIO;
            println!("Ripping AUDIO (MP3)");
        }
        else {
            // that's a path to the file to be examined
            file_paths.push(path::Path::new(&args[arg_index]));
        }

        arg_index += 1;
    }

    for file_path in file_paths {
        print!("\n");

        if !file_path.exists() {
            // does not exist
            println!("[ERROR] \"{}\" does not exist", file_path.display());
            continue;
        }

        // get file's metadata
        let file_metadata: std::fs::Metadata;
        match std::fs::metadata(file_path) {
            Ok(metadata) => {
                file_metadata = metadata;
            }

            Err(error) => {
                println!("[ERROR] Could not retrieve \"{}\"'s metadata: {}", file_path.display(), error);
                continue;
            }
        }

        // skip directories
        if file_metadata.is_dir() {
            println!("[INFO] Skipping directory \"{}\"...", file_path.display());
            continue;
        }

        // check if the file size is allowed
        if (file_metadata.len() as u128) > max_file_size {
            println!("[INFO] \"{}\" exceeds maximum file size. Skipping...", file_path.display());
            continue;
        }

        println!("[INFO] Working with \"{}\"...", file_path.display());

        let mut file_contents: Vec<u8> = Vec::with_capacity(file_metadata.len() as usize);

        // open file
        let mut file_handle: std::fs::File;
        match std::fs::File::open(file_path) {
            Ok(f_handle) => {
                file_handle = f_handle;
            }
            Err(error) => {
                println!("[ERROR] Could not open \"{}\": {}", file_path.display(), error);
                continue;
            }
        }

        // load into memory
        match file_handle.read_to_end(&mut file_contents) {
            Ok(_) => {}
            Err(error) => {
                println!("[ERROR] Error reading \"{}\": {}", file_path.display(), error);
            }
        }

        // keep track of found content
        let mut positions: Vec<Position> = Vec::new();

        match rip_type {
            RipType::IMG => {
                // find PNG positions
                let mut cursor_index: usize = 0;
                while (cursor_index as u64) < file_metadata.len() {
                    match rip_png(&file_contents, cursor_index) {
                        Some(pos) => {
                            cursor_index = pos.end;
                            positions.push(pos);
                        }
                        None => {
                            // no PNGs were found
                            break;
                        }
                    }
                }

                // find JPEG positions
                cursor_index = 0;
                while (cursor_index as u64) < file_metadata.len() {
                    match rip_jpeg(&file_contents, cursor_index) {
                        Some(pos) => {
                            cursor_index = pos.end;
                            positions.push(pos);
                        }
                        None => {
                            // no JPEGs were found
                            break;
                        }
                    }
                }
            }

            RipType::AUDIO => {
                // find MP3 positions
                let mut cursor_index = 0;
                while (cursor_index as u64) < file_metadata.len() {
                    match rip_mp3(&file_contents, cursor_index) {
                        Some(pos) => {
                            cursor_index = pos.end;
                            positions.push(pos);
                        }
                        None => {
                            // no MP3s were found
                            break;
                        }
                    }
                }

            }

            RipType::ALL => {
                // find PNG positions
                let mut cursor_index: usize = 0;
                while (cursor_index as u64) < file_metadata.len() {
                    match rip_png(&file_contents, cursor_index) {
                        Some(pos) => {
                            cursor_index = pos.end;
                            positions.push(pos);
                        }
                        None => {
                            // no PNGs were found
                            break;
                        }
                    }
                }

                // find JPEG positions
                cursor_index = 0;
                while (cursor_index as u64) < file_metadata.len() {
                    match rip_jpeg(&file_contents, cursor_index) {
                        Some(pos) => {
                            cursor_index = pos.end;
                            positions.push(pos);
                        }
                        None => {
                            // no JPEGs were found
                            break;
                        }
                    }
                }

                // find MP3 positions
                cursor_index = 0;
                while (cursor_index as u64) < file_metadata.len() {
                    match rip_mp3(&file_contents, cursor_index) {
                        Some(pos) => {
                            cursor_index = pos.end;
                            positions.push(pos);
                        }
                        None => {
                            // no MP3s were found
                            break;
                        }
                    }
                }
            }
        }


        if positions.len() == 0 {
            println!("[INFO] Didn't find anything");
            continue;
        }

        // get source filename to properly name the output files
        let source_file_name: String;
        match file_path.file_name() {
            Some(name) => {
                source_file_name = String::from(name.to_string_lossy());
            }
            None => {
                println!("[ERROR] Could not get \"{}\"'s filename", file_path.display());
                continue;
            }
        }

        // (TODO) work out overlaps
        // and save found files to the disk
        for position_index in 0..positions.len() {
            // create file
            let mut output_file_path_string: String = save_directory.join(&source_file_name).to_string_lossy().to_string();
            match positions[position_index].content_type {
                ContentType::PNG => {
                    output_file_path_string = 
                        output_file_path_string + &format!("_{}.png", position_index);
                }

                ContentType::JPEG => {
                    output_file_path_string =
                        output_file_path_string + &format!("_{}.jpeg", position_index);
                }

                ContentType::MP3 => {
                    output_file_path_string =
                        output_file_path_string + &format!("_{}.mp3", position_index);
                }
            }

            let mut output_file_handle: std::fs::File;
            match std::fs::File::create(&output_file_path_string) {
                Ok(f) => {
                    output_file_handle = f;
                }
                Err(error) => {
                    println!("[ERROR] Could not create output file \"{}\": {}", output_file_path_string, error);
                    continue;
                }
            }

            // write contents
            match output_file_handle.write(&file_contents[positions[position_index].start..positions[position_index].end]) {
                Ok(_) => {}
                Err(error) => {
                    println!("[ERROR] Error writing out the output file \"{}\": {}", output_file_path_string, error);
                }
            }

            println!("[INFO] Outputted {} ({} bytes)",
                output_file_path_string,
                positions[position_index].end - positions[position_index].start
            );
        }
    }
}
