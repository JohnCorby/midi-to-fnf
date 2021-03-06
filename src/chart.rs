//! holds chart format (kade engine but it should also work for vanilla)

use serde::*;
use serde_tuple::*;

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
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
    /// ex: stage
    pub stage: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
pub struct Section {
    pub sectionNotes: Vec<Note>,
    /// steps are those grid squares on the chart menu
    /// there are 4 steps per beat
    ///
    /// always 16
    pub lengthInSteps: u16,
    /// if true: player1 is notes 0-3 and player2 is 4-7.
    /// if false, the opposite.
    /// also controls camera (whoever has notes 0-3 is focused on)
    pub mustHitSection: bool,
}

#[derive(Serialize_tuple, Debug, Copy, Clone)]
pub struct Note {
    /// unit = ms
    pub time: f64,
    /// 0-7 representing arrows
    pub note: u8,
    /// unit = ms
    pub length: f64,
}
