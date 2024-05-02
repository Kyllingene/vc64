use nasm_rs;
use std::env;

fn main() {
    if env::var("CARGO_CFG_UNIX").is_ok() {
        if let Err(e) = nasm_rs::compile_library_args(
            "libunix_main.a",
            &["src/unix_main.asm"],
            &["-Psrc/sprites.inc", "-f elf64"],
        ) {
            // TODO: add better error handling
            panic!("{}", e);
        }
    } else if env::var("CARGO_CFG_WINDOWS").is_ok() {
        println!("cargo:rustc-link-lib=static=windows_main");
        if let Err(e) = nasm_rs::compile_library_args(
            "windows_main.lib",
            &["src/windows_main.asm"],
            &["-Psrc/sprites.inc", "-f win64"],
        ) {
            // TODO: add better error handling
            panic!("{}", e);
        }
    } else {
        panic!(
            "hit or miss? i guess {} never miss, huh?",
            env::var("CARGO_CFG_TARGET_OS").unwrap_or("they".into())
        )
    }
}
