use crate::keys;
use std::fs;
use std::io;

/// Writes a table of key counts to a file, most-pressed first.
pub fn save(counts: &[u64; 256], path: &str) -> io::Result<()> {
    // Collect only the keys that have a non-zero count.
    let mut rows: Vec<(usize, u64)> = Vec::new();
    for vk in 0..256 {
        if counts[vk] > 0 {
            rows.push((vk, counts[vk]));
        }
    }

    // Sort by count, highest first.
    rows.sort_by(|a, b| b.1.cmp(&a.1));

    let total: u64 = rows.iter().map(|&(_, c)| c).sum();

    // Build the whole table as one string, then write it to the file.
    let mut out = String::new();
    out.push_str(&format!("Key presses (total: {})\n", total));
    out.push_str(&format!("{:<14} {:>8}\n", "Key", "Count"));
    out.push_str(&"-".repeat(22));
    out.push('\n');
    for (vk, count) in &rows {
        out.push_str(&format!("{:<14} {:>8}\n", keys::name(*vk), count));
    }

    fs::write(path, out)
}
