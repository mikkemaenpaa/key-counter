mod keys;

use std::{thread, time::Duration};

#[link(name = "user32")]
unsafe extern "system" {
    fn GetAsyncKeyState(vkey: i32) -> i16;
}

const DELAY_TICKS: u32 = 27;
const REPEAT_TICKS: u32 = 2;

// no double counting for ctrl + shift + alt
fn should_skip(vk: usize) -> bool {
    matches!(vk, 0x00 | 0x03 | 0x07 | 0xA0..=0xA5)
}

fn main() {
    let mut counts = [0u64; 256];
    let mut down_ticks = [0u32; 256]; // how many ticks each key has been held

    loop {
        for vk in 0..256 {
            if should_skip(vk) {
                continue
            }
            let state = unsafe {GetAsyncKeyState(vk as i32) };
            let is_down = (state as u16 & 0x8000) != 0;

            if is_down {
                if down_ticks[vk] == 0 {
                    counts[vk] += 1;
                    println!("{} -> {}", keys::name(vk), counts[vk]);
                } else if !keys::no_repeat(vk) {
                    let held = down_ticks[vk];
                    if held >= DELAY_TICKS && (held - DELAY_TICKS) % REPEAT_TICKS == 0 {
                        counts[vk] += 1;
                        println!("{} -> {}", keys::name(vk), counts[vk]);
                    }
                }
                down_ticks[vk] += 1;
            } else {
                down_ticks[vk] = 0; 
            }
        }
        thread::sleep(Duration::from_millis(15));
    }
}