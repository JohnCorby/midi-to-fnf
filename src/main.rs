use midly::{Format, MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use rfd::FileDialog;
use std::collections::HashMap;
use std::env::current_dir;

mod data;
mod util;

fn main() {
    // get midi
    println!("select midi file you want to convert");
    let path = FileDialog::new()
        .add_filter("midi", &["mid"])
        .set_directory(current_dir().unwrap())
        .pick_file()
        .expect("no midi file picked");
    let data = std::fs::read(path).expect("error reading midi file");
    let midi = Smf::parse(&data).expect("error parsing midi file");

    // debugging lol
    println!("---------HEADER----------");
    println!("{:?}", midi.header);
    for (i, track) in midi.tracks.iter().enumerate() {
        println!("---------track {}----------", i);
        for event in track {
            println!("{:?}", event);
        }
    }

    assert_eq!(
        midi.header.format,
        Format::Parallel,
        "midi file should be parallel format"
    );
    assert_eq!(midi.tracks.len(), 2, "midi file should have 2 tracks");

    // get timing info
    let ticks_per_beat = match midi.header.timing {
        Timing::Metrical(ticks_per_beat) => ticks_per_beat,
        Timing::Timecode(_, _) => unimplemented!("time code timing"),
    };
    let beats_per_min = {
        let micros_per_beat = midi.tracks[0]
            .iter()
            .find_map(|event| match event.kind {
                TrackEventKind::Meta(MetaMessage::Tempo(micros_per_beat)) => {
                    Some(micros_per_beat.as_int())
                }
                _ => None,
            })
            .expect("couldn't find tempo in track 0");
        (60e6 / micros_per_beat as f64) as u16
    };
    println!("bpm = {}", beats_per_min);
    println!("ppq = {}", ticks_per_beat);

    // now get the notes
    let mut notes_pressed = HashMap::new();
    notes_pressed.insert(60u8, false);
    notes_pressed.insert(62, false);
    notes_pressed.insert(64, false);
    notes_pressed.insert(65, false);
    notes_pressed.insert(72, false);
    notes_pressed.insert(74, false);
    notes_pressed.insert(76, false);
    notes_pressed.insert(77, false);
    for &event in &midi.tracks[1] {
        let message = match event.kind {
            TrackEventKind::Midi { message, .. } => message,
            _ => continue,
        };

        let (note, pressed) = match message {
            MidiMessage::NoteOn { key, vel } if vel == 0 => (key.as_int(), false),
            MidiMessage::NoteOff { key, .. } => (key.as_int(), false),

            MidiMessage::NoteOn { key, .. } => (key.as_int(), true),

            _ => continue,
        };

        let was_pressed = match notes_pressed.get_mut(&note) {
            Some(was_pressed) => was_pressed,
            None => {
                // .unwrap_or_else(|| panic!("invalid note {}", note))
                eprintln!("invalid note {}", note);
                continue;
            }
        };
        assert_ne!(
            pressed, *was_pressed,
            "note on must be followed by note off and vice versa for each note"
        );

        *was_pressed = pressed;
    }
}
