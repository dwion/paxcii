//! Contains the `play_with_audio` method.

use crate::video::AsciiVideo;
use rodio::Decoder;
use std::{fs::File, io::BufReader};

impl AsciiVideo {
    /// Print the ascii video to stdout like `play` but alongside the specified audio file.\
    /// Supported audio formats: mp3/wav/ogg/flac/m4a/aac
    pub fn play_with_audio(self, file_path: &str) -> anyhow::Result<()> {
        let (_stream, handle) = rodio::OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&handle)?;

        // Open audio file
        let file = File::open(file_path)?;

        // Decode audio file
        let decoder = Decoder::new(BufReader::new(file))?;
        sink.append(decoder);

        // Play audio and ascii video
        sink.play();
        self.play();

        Ok(())
    }
}

