use nasm_rs;

fn main() {
	nasm_rs::compile_library("libasm_main.a", &["src/asm_main.asm"]).unwrap();
}