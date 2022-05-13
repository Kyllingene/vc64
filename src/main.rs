use macroquad::prelude::*;

// <platform>_main.asm must implement all of these functions
#[cfg(unix)]
#[link(name = "unix_main")]
extern "C" {
    fn update();
}

#[cfg(windows)]
#[link(name = "windows_main")]
extern "C" {
    fn update();
}

#[no_mangle]
pub extern "C" fn _draw_square(x: u64, y: u64, w: u64) {
    draw_rectangle(x as f32, y as f32, w as f32, w as f32, GREEN);
}

#[no_mangle]
pub extern "C" fn _clear(c: u64) {
    let color = match c {
        0 => BLACK,
        1 => WHITE,
        2 => GRAY,
        3 => GREEN,
        4 => BLUE,
        5 => RED,
        _ => DARKGRAY,
    };

    clear_background(color);
}

#[no_mangle]
pub extern "C" fn _key_down(k: u64) -> bool {
    let key = match k {
        0 => KeyCode::W,
        1 => KeyCode::A,
        2 => KeyCode::S,
        3 => KeyCode::D,
        4 => KeyCode::Z,
        5 => KeyCode::X,
        6 => KeyCode::C,
        7 => KeyCode::V,
        8 => KeyCode::Space,
        _ => panic!("Invalid key code: {}", k),
    };

    is_key_down(key)
}

#[no_mangle]
pub extern "C" fn _draw_rect(x: u64, y: u64, w: u64, h: u64, c: u64) {
    let color = match c {
        0 => BLACK,
        1 => WHITE,
        2 => GRAY,
        3 => GREEN,
        4 => BLUE,
        5 => RED,
        _ => DARKGRAY,
    };

    draw_rectangle(x as f32, y as f32, w as f32, h as f32, color);
}

#[macroquad::main("vc64")]
async fn main() {
    loop {
        unsafe {
            update();
        }

        next_frame().await
    }
}
