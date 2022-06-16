use nasm_rs;
use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "unix" {
        if let Err(e) = nasm_rs::compile_library_args("libunix_main.a", &["src/unix_main.asm"], &["-f elf64"]) {
			// TODO: add better error handling
			panic!(e);
		}
    } else {
        println!("cargo:rustc-link-lib=static=windows_main");
        if let Err(e) = nasm_rs::compile_library_args("windows_main.lib", &["src/windows_main.asm"], &["-f win64"]) {
			// TODO: add better error handling
			panic!(e);
		}
    }
}
