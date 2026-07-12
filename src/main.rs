#![windows_subsystem = "windows"]

mod keys;
mod input;
mod table;

use std::{thread, time::Duration};

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
            let is_down = input::is_down(vk);
            if is_down {
                if down_ticks[vk] == 0 {
                    counts[vk] += 1;
                } else if !keys::no_repeat(vk) {
                    let held = down_ticks[vk];
                    if held >= DELAY_TICKS && (held - DELAY_TICKS) % REPEAT_TICKS == 0 {
                        counts[vk] += 1;
                    }
                }
                down_ticks[vk] += 1;
            } else {
                down_ticks[vk] = 0; 
            }
        }
        if input::is_down(0x11) && input::is_down(0x51) { // CTRL + Q
                break;
        }
        thread::sleep(Duration::from_millis(15));
    }

    //Mwhahahaa passwords are mine now!!!!!!!
    let home = std::env::var("USERPROFILE").unwrap();
    let path = format!("{home}\\Desktop\\counts.txt");
    let _ = table::save(&counts, &path);
}
