use crate::util::ticks_to_millis;
use midly::{Format, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};
use rfd::FileDialog;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::stdin;

mod chart;
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

    // get timing info
    let ticks_per_beat = match midi.header.timing {
        Timing::Metrical(ticks_per_beat) => ticks_per_beat,
        Timing::Timecode(_, _) => unimplemented!("time code timing"),
    }
    .as_int();
    let bpm = midi.tracks[0]
        .iter()
        .find_map(|event| match event.kind {
            TrackEventKind::Meta(MetaMessage::Tempo(micros_per_beat)) => {
                Some(micros_per_beat.as_int())
            }
            _ => None,
        })
        .map(|micros_per_beat| (60e6 / micros_per_beat as f64) as u16)
        .unwrap_or_else(|| {
            println!("couldn't find bpm in track 0");
            println!("please input it manually");
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            line.trim().parse().expect("could not parse bpm")
        });
    println!("bpm = {}", bpm);
    println!("ppq = {}", ticks_per_beat);

    let notes_track = match midi.header.format {
        // ableton uses this
        Format::SingleTrack => &midi.tracks[0],

        // fl studio uses this
        Format::Parallel => {
            assert_eq!(
                midi.tracks.len(),
                2,
                "midi file in parallel format must have 2 tracks"
            );
            &midi.tracks[1]
        }

        Format::Sequential => unimplemented!("sequential format not supported"),
    };

    let chart_notes = get_chart_notes(notes_track, ticks_per_beat, bpm);
    // debugging lol
    for chart_note in chart_notes {
        println!("{:?}", chart_note);
    }
}

/// turn midi events into chart notes
fn get_chart_notes(notes_track: &[TrackEvent], ticks_per_beat: u16, bpm: u16) -> Vec<chart::Note> {
    // now get the notes
    let mut chart_notes = vec![];

    let mut notes_state = HashMap::new();
    let init_state = (false, 0.);
    notes_state.insert(60, init_state);
    notes_state.insert(62, init_state);
    notes_state.insert(64, init_state);
    notes_state.insert(65, init_state);
    notes_state.insert(72, init_state);
    notes_state.insert(74, init_state);
    notes_state.insert(76, init_state);
    notes_state.insert(77, init_state);

    let mut time = 0.;

    for &event in notes_track {
        time += ticks_to_millis(event.delta.as_int(), ticks_per_beat, bpm);

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

        let (was_pressed, last_time) = match notes_state.get_mut(&note) {
            Some(state) => state,
            None => {
                // .unwrap_or_else(|| panic!("invalid note {}", note))
                eprintln!("ignoring invalid note {}", note);
                continue;
            }
        };
        assert_ne!(
            pressed, *was_pressed,
            "note on must be followed by note off and vice versa for each note"
        );

        if !pressed {
            let length = time - *last_time;
            chart_notes.push(chart::Note { time, note, length })
        }

        *was_pressed = pressed;
        *last_time = time;
    }
    for (note, (pressed, _)) in notes_state {
        assert!(!pressed, "note {} never got a final note off event", note)
    }

    chart_notes
}
