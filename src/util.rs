pub fn ticks_to_steps(ticks: u32, ticks_per_beat: u16) -> f64 {
    let beats = ticks as f64 / ticks_per_beat as f64;
    beats * 4.
}

pub fn steps_to_millis(steps: f64, bpm: u16) -> f64 {
    let millis_per_beat = 60e3 / bpm as f64;
    let millis_per_step = millis_per_beat / 4.;
    steps * millis_per_step
}
