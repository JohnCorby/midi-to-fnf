use midly::{Format, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};
use rfd::FileDialog;
use std::collections::HashMap;

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
            util::prompt(
                "couldn't find bpm in track 0\nplease input it manually",
                None,
            )
            .parse()
            .expect("could not parse bpm")
        });
    println!("bpm = {}", bpm);
    println!("ppq = {}", ticks_per_beat);

    let notes = get_chart_notes(notes_track, ticks_per_beat);

    // put the notes into sections
    const SECTION_LEN: u16 = 16;
    let mut sections = vec![chart::Section {
        sectionNotes: Vec::with_capacity(SECTION_LEN as usize),
        lengthInSteps: SECTION_LEN,
        mustHitSection: false,
    }];
    for mut note in notes {
        // we are past the section's range, make new sections until we're not
        while note.time >= sections.len() as f64 * SECTION_LEN as f64 {
            sections.push(chart::Section {
                sectionNotes: Vec::with_capacity(SECTION_LEN as usize),
                lengthInSteps: SECTION_LEN,
                mustHitSection: false,
            })
        }

        note.time = util::steps_to_millis(note.time, bpm);
        note.length = util::steps_to_millis(note.length, bpm);
        println!("{:?}", note);
        sections.last_mut().unwrap().sectionNotes.push(note);
    }

    // whoever has more notes gets the camera focused on them
    for section in &mut sections {
        if section.sectionNotes.is_empty() {
            continue;
        }
        let mut player1 = 0;
        let mut player2 = 0;
        for &note in &section.sectionNotes {
            match note.note {
                4..=7 => player1 += 1,
                0..=3 => player2 += 1,
                _ => unreachable!(),
            }
        }
        if player1 >= player2 {
            section.mustHitSection = true;
            for note in &mut section.sectionNotes {
                note.note = match note.note {
                    4..=7 => note.note - 4,
                    0..=3 => note.note + 4,
                    _ => unreachable!(),
                }
            }
        }
    }

    // make the song and save it
    let song = util::prompt("provide a song name", Some("pico"));
    let speed = util::prompt("provide song speed", Some("3"))
        .parse()
        .expect("could not parse song speed");
    let player1 = util::prompt("provide player 1 character", Some("bf"));
    let player2 = util::prompt("provide player 2 character", Some("pico"));
    let stage = util::prompt("provide stage", Some("philly"));
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

    println!("provide the json file to save");
    let path = FileDialog::new()
        .add_filter("json", &["json"])
        .save_file()
        .expect("no json file given");
    std::fs::write(path, format!("{:#}", json)).expect("error writing to json file");
}

/// turn midi events into chart notes
fn get_chart_notes(notes_track: &[TrackEvent], ticks_per_beat: u16) -> Vec<chart::Note> {
    let len_beats_threshold = util::prompt(
        "provide note length threshold in beats\nnotes shorter than this will have no trail",
        Some("0.5"),
    )
    .parse::<f64>()
    .expect("could not parse note length threshold");

    let mut chart_notes = vec![];

    let mut notes_state = HashMap::new();
    notes_state.insert(60, (4, false, 0.));
    notes_state.insert(61, (5, false, 0.));
    notes_state.insert(62, (6, false, 0.));
    notes_state.insert(63, (7, false, 0.));
    notes_state.insert(72, (0, false, 0.));
    notes_state.insert(73, (1, false, 0.));
    notes_state.insert(74, (2, false, 0.));
    notes_state.insert(75, (3, false, 0.));

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

        let (note, was_pressed, last_time) = notes_state
            .get_mut(&note)
            .unwrap_or_else(|| panic!("invalid note {}", note));
        assert_ne!(
            pressed, *was_pressed,
            "note on must be followed by note off and vice versa for note {} time {}",
            note, time
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
    for (note, (_, pressed, _)) in notes_state {
        assert!(!pressed, "note {} never got a final note off event", note)
    }

    chart_notes
}
