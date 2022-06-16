# vc64

## Description
*vc64* is a just-for-fun project made in Rust. It uses [macroquad](https://crates.io/crates/macroquad) to make a cross-platform virtual console of sorts that you program in (NASM) x86-64 assembly.
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

## Usage
The current implementation is  ~100 lines and easy to read, so documentation would be superfluous at this stage. However, `src/main.asm` holds an example program to familiarize you with the concepts.

There are two assembly source files: `src/windows_main.asm` and `src/unix_main.asm`. This is necessary since Windows and Unix use different calling conventions.
The build script will automatically assemble the correct source file for your target OS, but you must manually tranlate between the two.
***To build, you MUST have `nasm` in your path!***

### Building
#### Unix
Make sure you have Rust installed. Clone the repository once per game, then `cd` into the folder and run `cargo build`.
This will produce a standalone executable in `target/debug/`. That is your game.

#### Windows
Make sure you have Rust and Visual Studio installed, along with VS' C++ tools. Then you can run `cargo build` as usual.
A common indicator that your Windows assembly source is using the wrong calling convention is if it immediately panics with the text "Invalid key code: <some number>".
