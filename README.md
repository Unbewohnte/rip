# RIP
## various embedded content

# Use
`rip -h` will print out this help message

`
rip (optional)[FLAG]... (optional; default=ALL)[RIPTYPE] [FILE]...

                
[FLAG]s
"-v"   or "--version"               -> print version
"-h"   or "--help"                  -> print this message
"-sd"  or "--save-dir" [DIR]        -> specify save directory
"-mfs" or "--max-file-size" [SIZE]  -> skip files bigger than size (in bytes)

                
[RIPTYPE]
ALL   -> rip everything that seems like an embedded content
IMG   -> try to look for images only
AUDIO -> rip audio content
`

### Examples
- `rip audio music/*` -> extract found audio data from all files in music directory
- `rip -sd extracted img game_with_cool_sprites.exe` -> get image data from `game_with_cool_sprites.exe` and save it to `extracted` folder
- `rip -mfs 52428800 all various_files/*` -> rip everything from files that are under 50MB
- `rip all various_files/file1.data various_files/file2.xp3` -> rip everything from file1.data and file2.xp3

# Compile
As usual - `cargo build --release` or simply `make all` if you have it.

RIP has no dependencies and never will so no Internet connection needed in order to build it

# License
RIP is under GPLv3