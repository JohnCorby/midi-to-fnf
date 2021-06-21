use std::io::stdin;

pub fn ticks_to_steps(ticks: u32, ticks_per_beat: u16) -> f64 {
    let beats = ticks as f64 / ticks_per_beat as f64;
    beats * 4.
}

pub fn steps_to_millis(steps: f64, bpm: u16) -> f64 {
    let millis_per_beat = 60e3 / bpm as f64;
    let millis_per_step = millis_per_beat / 4.;
    steps * millis_per_step
}

/// get line input
pub fn prompt(prompt: &str, default: Option<&str>) -> String {
    if let Some(default) = default {
        println!("{}\n(leave blank for default = \"{}\")", prompt, default)
    } else {
        println!("{}", prompt)
    }
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    if line.is_empty() {
        default.expect("a value is required").to_string()
    } else {
        line
    }
}
