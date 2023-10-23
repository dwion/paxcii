// Characters used to display ASCII output
/// `[' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8']`
pub const CHARS_LIGHT: [char; 11] = [' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8'];
/// `['.', '*', 'e', 's', '◍']`
pub const CHARS_MEDIUM: [char; 5] = ['.', '*', 'e', 's', '◍'];
/// `['░', '▒', '▓', '█']`
pub const CHARS_FILLED: [char; 4] = ['░', '▒', '▓', '█'];

#[derive(Debug, Clone, PartialEq)]
/// Settings used when converting image or video to ascii.
pub struct PaxciiSettings {
    pub color: bool,
    pub char_set: Vec<char>,
    pub width: u32,
    pub height: u32,
    pub keep_aspect_ratio: bool,
}

impl PaxciiSettings {
    /// Changes `char_set` to [`CHARS_LIGHT`]
    pub fn chars_light(&mut self) {
        self.char_set = Vec::from(CHARS_LIGHT)
    }
    /// Changes `char_set` to [`CHARS_MEDIUM`]
    pub fn chars_medium(&mut self) {
        self.char_set = Vec::from(CHARS_MEDIUM)
    }
    /// Changes `char_set` to [`CHARS_FILLED`]
    pub fn chars_filled(&mut self) {
        self.char_set = Vec::from(CHARS_FILLED)
    }
}

impl Default for PaxciiSettings {
    fn default() -> PaxciiSettings {
        PaxciiSettings {
            color: true,
            char_set: Vec::from(CHARS_MEDIUM),
            width: 30,
            height: 30,
            keep_aspect_ratio: true,
        }
    }
}
