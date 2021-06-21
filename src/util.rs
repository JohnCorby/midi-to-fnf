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

pub fn init_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let message = if let Some(message) = info.message() {
            message.to_string()
        } else if let Some(payload) = info.payload().downcast_ref::<&str>() {
            payload.to_string()
        } else {
            "cannot get panic message".to_string()
        };

        eprintln!("Error: {}", message);
        eprintln!("press enter to exit");
        stdin().read_line(&mut String::new()).unwrap();
    }))
}
