use crate::img::CHARS_MEDIUM;

#[derive(Debug, Clone, PartialEq)]
/// Settings used when converting image or video to ascii.
pub struct PaxciiSettings {
    pub fps: u64,
    pub color: bool,
    pub char_set: Vec<char>,
    pub width: u32,
    pub height: u32,
    pub preserve_aspect_ratio: bool,
}

impl Default for PaxciiSettings {
    fn default() -> PaxciiSettings {
        PaxciiSettings {
            fps: 30,
            color: true,
            char_set: Vec::from(CHARS_MEDIUM),
            width: 30,
            height: 30,
            preserve_aspect_ratio: true,
        }
    }
}