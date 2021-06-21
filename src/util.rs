pub fn ticks_to_millis(ticks: u32, ticks_per_beat: u16, bpm: u16) -> f64 {
    let beats = ticks as f64 / ticks_per_beat as f64;
    let millis_per_beat = 60e3 / bpm as f64;
    beats * millis_per_beat
}

// pub fn round_decimals_up(number: f64, decimals: u32) -> i64 {
//     let factor = 10i64.pow(decimals);
//     (number * factor as f64).ceil() as i64 / factor
// }
