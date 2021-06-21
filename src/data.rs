//! holds vanilla and kade engine song json formats
//!
//! actually, ill just use vanilla for now

use serde::*;
use serde_tuple::*;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Song {
    song: String,
    notes: Vec<Section>,
    bpm: u16,
    /// if there is a voice track
    needsVoices: bool,
    speed: f64,

    /// ex: bf
    player1: String,
    /// ex: dad
    player2: String,
    /// always true
    validScore: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Section {
    sectionNotes: Vec<Note>,
    /// steps are those grid squares on the chart menu
    /// there are 4 steps per beat
    ///
    /// always 16
    lengthInSteps: u16,
    /// always 0
    typeOfSection: u8,
    /// if true: player1 is notes 0-3 and player2 is 4-7.
    /// if false, the opposite.
    /// also controls camera (whoever has notes 0-3 is focused on)
    mustHitSection: bool,
    bpm: u16,
    changeBPM: bool,
    altAnim: bool,
}

#[derive(Debug, Serialize_tuple)]
pub struct Note {
    /// unit = ms
    time: f64,
    /// 0-7 representing arrows
    note: u8,
    /// unit = ms
    length: f64,
}
