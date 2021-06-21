//! holds vanilla and kade engine song json formats
//!
//! actually, ill just use vanilla for now

use serde::*;
use serde_tuple::*;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Song {
    pub song: String,
    pub notes: Vec<Section>,
    pub bpm: u16,
    /// if there is a voice track
    pub needsVoices: bool,
    pub speed: f64,

    /// ex: bf
    pub player1: String,
    /// ex: dad
    pub player2: String,
    /// always true
    pub validScore: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Section {
    pub sectionNotes: Vec<Note>,
    /// steps are those grid squares on the chart menu
    /// there are 4 steps per beat
    ///
    /// always 16
    pub lengthInSteps: u16,
    /// always 0
    pub typeOfSection: u8,
    /// if true: player1 is notes 0-3 and player2 is 4-7.
    /// if false, the opposite.
    /// also controls camera (whoever has notes 0-3 is focused on)
    pub mustHitSection: bool,
    pub bpm: u16,
    pub changeBPM: bool,
    pub altAnim: bool,
}

#[derive(Debug, Serialize_tuple)]
pub struct Note {
    /// unit = ms
    pub time: f64,
    /// 0-7 representing arrows
    pub note: u8,
    /// unit = ms
    pub length: f64,
}
