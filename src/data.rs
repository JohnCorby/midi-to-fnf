//! holds vanilla and kade engine song json formats
//!
//! actually, ill just use kade engine format for now and hope it works on vanilla as well :P

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Song {
    song: String,
    notes: Vec<Section>,
    player1: String,
    player2: String,
    stage: String,
    /// im assuming this is for if there neesd to be a voices.ogg?
    needsVoices: bool,
    /// todo wtf is this
    validScore: bool,
    bpm: u16,
    speed: f32,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Section {
    sectionNotes: Vec<Note>,
    /// todo wtf is a step?
    lengthInSteps: u16,
    mustHitSection: bool,
    /// todo wtf is this
    altAnim: bool,
    /// todo wtf is this, always 0?
    typeOfSection: u8,
    /// bruh
    bpm: u16,
    changeBPM: bool,
}

/// todo this is represented as an array, how do we do that????
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Note {
    /// todo what unit? it's not seconds
    time: f32,
    /// 0-3 representing arrows
    note: u8,
    /// todo what unit?
    length: u16,
}
