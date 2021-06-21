use midly::{Format, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};
use rfd::FileDialog;
use std::collections::HashMap;
use std::io::stdin;

mod chart;
mod util;

fn main() {
    // get midi
    println!("select midi file you want to convert");
    let path = FileDialog::new()
        .add_filter("midi", &["mid"])
        .pick_file()
        .expect("no midi file picked");
    let data = std::fs::read(path).expect("error reading midi file");
    let midi = Smf::parse(&data).expect("error parsing midi file");

    // debugging lol
    // println!("---------HEADER----------");
    // println!("{:?}", midi.header);
    // for (i, track) in midi.tracks.iter().enumerate() {
    //     println!("---------track {}----------", i);
    //     for event in track {
    //         println!("{:?}", event);
    //     }
    // }

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

    let notes = get_chart_notes(notes_track, ticks_per_beat);

    // put the notes into sections
    const SECTION_LEN: u16 = 16;
    let mut sections = vec![chart::Section {
        sectionNotes: Vec::with_capacity(SECTION_LEN as usize),
        lengthInSteps: SECTION_LEN,
        mustHitSection: true,
    }];
    for mut note in notes {
        // we are past the section's range, get us back into it
        while note.time >= (sections.len() as f64 + 1.) * SECTION_LEN as f64 {
            sections.push(chart::Section {
                sectionNotes: Vec::with_capacity(SECTION_LEN as usize),
                lengthInSteps: SECTION_LEN,
                mustHitSection: true,
            })
        }

        note.time = util::steps_to_millis(note.time, bpm);
        note.length = util::steps_to_millis(note.length, bpm);
        println!("{:?}", note);
        sections.last_mut().unwrap().sectionNotes.push(note);
    }

    // make the song and save it
    let song = "Bopeebo".to_string();
    let speed = 3.;
    let player1 = "bf".to_string();
    let player2 = "dad".to_string();
    let stage = "stage".to_string();
    let song = chart::Song {
        song,
        notes: sections,
        bpm,
        needsVoices: true,
        speed,

        player1,
        player2,
        stage,
    };
    let json = serde_json::json!({ "song": song });
    // debugging lol
    println!("{:#}", json);

    println!("provide the json file to save");
    let path = FileDialog::new()
        .add_filter("json", &["json"])
        .save_file()
        .expect("no json file given");
    std::fs::write(path, format!("{:#}", json)).expect("error writing to json file");
}

/// turn midi events into chart notes
fn get_chart_notes(notes_track: &[TrackEvent], ticks_per_beat: u16) -> Vec<chart::Note> {
    let len_beats_threshold = 0.5;

    let mut chart_notes = vec![];

    let mut notes_state = HashMap::new();
    let init_state = (false, 0.);
    notes_state.insert(60, (0, init_state));
    notes_state.insert(61, (1, init_state));
    notes_state.insert(62, (2, init_state));
    notes_state.insert(63, (3, init_state));
    notes_state.insert(72, (4, init_state));
    notes_state.insert(73, (5, init_state));
    notes_state.insert(74, (6, init_state));
    notes_state.insert(75, (7, init_state));

    let mut time = 0.;

    for &event in notes_track {
        time += util::ticks_to_steps(event.delta.as_int(), ticks_per_beat);

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

        let (note, (was_pressed, last_time)) = match notes_state.get_mut(&note) {
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
            let mut note = chart::Note {
                time: *last_time,
                note: *note,
                length: time - *last_time,
            };
            // truncate short len notes to 0
            if note.length <= len_beats_threshold * 4. {
                note.length = 0.
            }
            chart_notes.push(note)
        }

        *was_pressed = pressed;
        *last_time = time;
    }
    for (note, (_, (pressed, _))) in notes_state {
        assert!(!pressed, "note {} never got a final note off event", note)
    }

    chart_notes
}
