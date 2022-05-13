use bit_array::BitArray;
use macroquad::prelude::*;
use typenum::U8;

// <platform>_main.asm must implement all of these functions
#[cfg(unix)]
#[link(name = "unix_main")]
extern "C" {
    fn update(dt: f64);
}

#[cfg(windows)]
#[link(name = "windows_main")]
extern "C" {
    fn update(dt: f64);
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

#[no_mangle]
pub extern "C" fn _draw_px(x: u64, y: u64, p: bool) {
    if p {
        let px = x / 4 * 4;
        let py = y / 4 * 4;
        draw_rectangle(px as f32, py as f32, 4.0, 4.0, BLACK);
    }
}

fn draw_u8(x: u64, y: u64, u: u8) {
    let bv = BitArray::<u16, U8>::from_bytes(&[u]);

    let mut px = x;
    for p in bv.iter() {
        _draw_px(px, y, p);

        px += 4;
    }
}

#[no_mangle]
pub extern "C" fn _draw_sprite(x: u64, y: u64, s: *mut [u8; 32]) {
    let sprite = unsafe { <*mut [u8; 32]>::as_ref(s) }.unwrap().to_vec(); // turns the pointer to an array into a vec

    let mut p = 0;
    let mut px = x;
    let mut py = y;
    for u in &sprite {
        draw_u8(px, py, *u);
        p += 1;

        if p == 1 {
            px += 32;
        }

        if p == 2 {
            p = 0;
            px -= 32;
            py += 4;
        }
    }
}

#[macroquad::main("vc64")]
async fn main() {
    loop {
        let dt = get_frame_time() as f64;

        unsafe {
            update(dt);
        }

        next_frame().await
    }
}
