#![allow(dead_code)]

use std::time::Duration;

pub fn ticks_to_secs(ticks: u64, tempo: f64, ticks_per_beat: u64) -> Duration {
    let secs = ticks as f64 / ticks_per_beat as f64 * tempo;
    Duration::from_secs_f64(secs)
}

pub fn note_to_freq(note: u8) -> f64 {
    const A_FREQ: f64 = 440.;
    (A_FREQ / 32.) * 2f64.powf((note as f64 - 9.) / 12.)
}

pub fn round_decimals_up(number: f64, decimals: u32) -> i64 {
    let factor = 10i64.pow(decimals);
    (number * factor as f64).ceil() as i64 / factor
}
