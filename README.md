# vc64

## Description
*vc64* is a just-for-fun project made in Rust. It uses macroquad to make a cross-platform\* virtual console of sorts that you program in x86-64 assembly.
In it's current state, it only supports the following features:
 - 7 colors
 - clear the screen
 - get state of 9 keys
 - draw rectangles (currently uses `u64` instead of `f32`, so poor precision)
 
However, the hope is for these features to be implemented by the end:
 - 16x16 sprite table (written in `src/sprites.asm`)
 - 4-color pallette with 32 available colors
 - Dynamically-linked object files instead of only statically-linked files
 - TBD

###### \*: currently only supports Unix or WSL. WSL is great, install it. Any help on native Windows support would be welcome.

## Usage
The current implementation is  ~100 lines and easy to read, so documentation would be superfluous at this stage. However, `src/main.asm` holds an example program to familiarize you with the concepts.

### Building
Make sure you have Rust installed. Clone the repository once per game, then `cd` into the folder and run:
```sh
cargo build
```

This will produce a standalone executable in `target/debug/`. That is your game.